use crate::{
    i18n::{t, tf},adb::Adb, process};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub const TERMS: &str = "https://developer.android.com/tools/releases/platform-tools#downloads";
pub const VERSION: &str = "37.0.1";
/// 공식 Platform-Tools 압축 파일(8~16MiB)과 그 안의 파일 하나가 넘을 수 없는 상한.
const ARCHIVE_LIMIT: u64 = 64 * 1024 * 1024;
/// 다운로드 전체 예산. ureq의 global 예산은 본문 수신까지 포함하므로 저속 회선을 고려해 넉넉히 둔다.
/// 취소는 청크마다 검사하지만 멈춘 읽기는 이 예산까지 기다릴 수 있다.
const DOWNLOAD_BUDGET: Duration = Duration::from_secs(90);
#[derive(Default, Serialize, Deserialize)]
struct Settings {
    adb: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}
fn read_settings() -> Settings {
    let Ok(dir) = data_dir() else { return Settings::default() };
    let path = dir.join("settings.json");
    match fs::metadata(&path) {
        Ok(meta) if meta.len() <= 16384 => fs::read(&path)
            .ok()
            .and_then(|bytes| {
                // 편집기가 붙인 UTF-8 BOM은 JSON이 아니므로 걷어내고 읽는다.
                let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(&bytes);
                serde_json::from_slice(bytes).ok()
            })
            .unwrap_or_default(),
        _ => Settings::default(),
    }
}
fn write_settings(settings: &Settings, failure: &str) -> Result<(), String> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir).map_err(|_| t("설정 폴더를 만들지 못했습니다."))?;
    let temp = dir.join(format!("settings-{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec(settings).map_err(|_| t("설정 변환 실패"))?;
    fs::write(&temp, bytes).map_err(|_| t(failure))?;
    fs::rename(&temp, dir.join("settings.json")).map_err(|_| t(failure))?;
    Ok(())
}
/// 저장된 앱 언어 코드. 없으면 None(기기 언어를 따른다).
pub fn saved_language() -> Option<String> {
    read_settings().lang
}
/// 앱 언어를 저장한다. ADB 위치 설정은 그대로 둔다.
pub fn save_language(code: &str) -> Result<(), String> {
    let mut settings = read_settings();
    settings.lang = Some(code.to_string());
    write_settings(&settings, "언어 설정을 저장하지 못했습니다.")
}
pub fn data_dir() -> Result<PathBuf, String> {
    // 개발자용 스위치: 데이터 폴더를 다른 곳으로 돌려 "처음 실행" 상태를 실제 데이터를 건드리지 않고 재현한다.
    if let Some(dir) = std::env::var_os("ADB_ON_DATA_DIR").filter(|v| !v.is_empty()) {
        return Ok(PathBuf::from(dir));
    }
    ProjectDirs::from("ai", "tinygem", "adb-on")
        .map(|d| d.data_local_dir().to_owned())
        .ok_or_else(|| t("사용자 데이터 폴더를 찾지 못했습니다."))
}
/// 사용자에게 보여 주는 데이터 폴더 경로. 폴더가 아직 없어도 위치는 정해져 있다.
pub fn data_dir_display() -> String {
    data_dir().map(|d| display_path(&d)).unwrap_or_default()
}
/// 화면에 보이는 경로. 폴더가 있으면 디스크에 적힌 대소문자 그대로 보여 준다(Windows는 대소문자를 구분하지
/// 않아 설정에 저장된 경로와 기본 폴더 이름의 표기가 서로 다를 수 있다). `\\?\` 접두사는 뗀다.
pub fn display_path(path: &Path) -> String {
    let shown = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()).display().to_string();
    shown.strip_prefix(r"\\?\").map(str::to_string).unwrap_or(shown)
}
/// 앱이 준비한 adb가 프로세스로 실행 중인지 본다. Windows는 실행 중인 파일을 쓰기로 열 수 없고,
/// Unix는 실행 중이어도 삭제가 막히지 않으므로 검사할 필요가 없다.
fn own_adb_running(own: &Path) -> bool {
    if !own.is_file() {
        return false;
    }
    if cfg!(windows) {
        fs::OpenOptions::new().write(true).open(own).is_err()
    } else {
        false
    }
}
/// 앱이 만든 사용자 데이터(설정, 준비한 platform-tools, 옆으로 옮긴 손상본)를 모두 지운다.
/// 여기서 준비한 adb가 서버로 실행 중이면 그 서버만 종료한 뒤 지운다. 다른 위치의 ADB,
/// 사용자 SDK, ADB 인증키는 대상이 아니다.
pub fn remove_data(stop: &Arc<AtomicBool>) -> Result<(), String> {
    let dir = data_dir()?;
    if !dir.exists() {
        return Ok(());
    }
    let own = dir.join(format!("platform-tools-{VERSION}")).join(executable());
    if own_adb_running(&own) {
        // 잠금은 준비본 adb가 서버로 떠 있다는 뜻이다. 그 바이너리로 기본 서버를 종료하고
        // 프로세스가 실제로 끝나 파일이 풀릴 때까지 기다린다.
        let _ = process::run(&own, &["kill-server"], None, stop, 5);
        let deadline = Instant::now() + Duration::from_secs(3);
        while own_adb_running(&own) && Instant::now() < deadline && !stop.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }
    }
    // 핸들 해제는 종료 직후 잠시 늦을 수 있어 짧게 재시도한다.
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut attempt = fs::remove_dir_all(&dir);
    while attempt.is_err() && Instant::now() < deadline && !stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
        attempt = fs::remove_dir_all(&dir);
    }
    attempt.map_err(|_| {
        tf(
            "지우지 못했어요. 다른 프로그램이 이 폴더의 파일을 사용 중일 수 있어요. 창을 닫고 직접 지워 주세요.\n{}",
            &[&dir.display().to_string()],
        )
    })?;
    prune_parents(&dir);
    Ok(())
}
/// 화면에 함께 보여 줄 ADB의 출처. 어디서 온 도구인지 알아야 "데이터 삭제" 뒤에도 경로가 보이는 이유를 이해할 수 있다.
/// 출처 종류. 1 = adb-on이 준비한 도구, 2 = 직접 선택, 3 = Android SDK, 4 = PATH. UI는 1일 때 준비 버튼을 잠근다.
pub fn describe_source(path: &Path) -> (i32, String) {
    let own = data_dir()
        .map(|d| d.join(format!("platform-tools-{VERSION}")).join(executable()))
        .ok();
    let same = |a: &Path, b: &Path| {
        fs::canonicalize(a).ok().zip(fs::canonicalize(b).ok()).map(|(a, b)| a == b).unwrap_or(a == b)
    };
    if own.as_deref().is_some_and(|o| same(o, path)) {
        return (1, t("adb-on이 준비한 Google 공식 도구"));
    }
    if read_settings().adb.as_deref().is_some_and(|s| same(s, path)) {
        return (2, t("직접 선택한 파일"));
    }
    let sdk = ["ANDROID_HOME", "ANDROID_SDK_ROOT"]
        .iter()
        .filter_map(|v| std::env::var_os(v))
        .map(|v| PathBuf::from(v).join("platform-tools").join(executable()))
        .chain(BaseDirs::new().map(|b| {
            if cfg!(windows) {
                b.data_local_dir().join("Android/Sdk/platform-tools/adb.exe")
            } else if cfg!(target_os = "macos") {
                b.home_dir().join("Library/Android/sdk/platform-tools/adb")
            } else {
                b.home_dir().join("Android/Sdk/platform-tools/adb")
            }
        }))
        .any(|c| same(&c, path));
    if sdk {
        (3, t("이 PC의 Android SDK"))
    } else {
        (4, t("PATH에서 찾은 도구"))
    }
}
/// 데이터 폴더의 상위(`…\tinygem\adb-on`, `…\tinygem`)는 앱이 만든 것일 때만, 비어 있을 때만 지운다.
fn prune_parents(dir: &Path) {
    let mut current = dir.parent();
    while let Some(parent) = current {
        match parent.file_name().and_then(|n| n.to_str()) {
            Some("adb-on") | Some("tinygem") => {
                if fs::remove_dir(parent).is_err() {
                    break;
                }
            }
            _ => break,
        }
        current = parent.parent();
    }
}
pub fn executable() -> &'static str {
    if cfg!(windows) { "adb.exe" } else { "adb" }
}
pub fn select(path: &Path, stop: &Arc<AtomicBool>) -> Result<Adb, String> {
    let adb = Adb::inspect(path, stop)?;
    let mut settings = read_settings();
    settings.adb = Some(adb.path.clone());
    write_settings(&settings, "ADB 위치를 저장하지 못했습니다.")?;
    Ok(adb)
}
pub fn locate(stop: &Arc<AtomicBool>) -> Result<Option<Adb>, String> {
    let mut candidates = Vec::new();
    if let Some(path) = read_settings().adb {
        candidates.push(path);
    }
    // 개발자용 스위치: SDK·PATH의 ADB를 못 본 척해 "도구 없음" 상태를 재현한다. 저장된 선택과 앱이 준비한 도구는 그대로 본다.
    let ignore_system = std::env::var_os("ADB_ON_IGNORE_SYSTEM_ADB").is_some_and(|v| v == "1");
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if ignore_system {
            break;
        }
        if let Some(value) = std::env::var_os(var) {
            candidates.push(
                PathBuf::from(value)
                    .join("platform-tools")
                    .join(executable()),
            );
        }
    }
    if let Some(base) = BaseDirs::new().filter(|_| !ignore_system) {
        #[cfg(windows)]
        candidates.push(
            base.data_local_dir()
                .join("Android/Sdk/platform-tools/adb.exe"),
        );
        #[cfg(target_os = "macos")]
        candidates.push(
            base.home_dir()
                .join("Library/Android/sdk/platform-tools/adb"),
        );
        #[cfg(target_os = "linux")]
        candidates.push(base.home_dir().join("Android/Sdk/platform-tools/adb"));
    }
    if let Some(path) = std::env::var_os("PATH").filter(|_| !ignore_system) {
        for dir in std::env::split_paths(&path) {
            // 현재 작업 폴더의 우연한 실행 파일은 자동 선택하지 않는다.
            if dir.is_absolute() {
                candidates.push(dir.join(executable()));
            }
        }
    }
    if let Ok(dir) = data_dir() {
        candidates.push(
            dir.join(format!("platform-tools-{VERSION}"))
                .join(executable()),
        );
    }
    for path in candidates {
        if stop.load(Ordering::Relaxed) {
            return Err(t("작업을 취소했습니다."));
        }
        if path.is_file()
            && let Ok(adb) = Adb::inspect(&path, stop)
        {
            return Ok(Some(adb));
        }
    }
    Ok(None)
}

pub fn install(consented: bool, stop: &Arc<AtomicBool>) -> Result<Adb, String> {
    if !consented {
        return Err(t("Google SDK 약관 동의가 필요합니다."));
    }
    let platform = if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        return Err(t("자동 준비는 Windows와 macOS에서 지원합니다."));
    };
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../scripts/platform-tools.json"))
            .map_err(|_| t("다운로드 정보 오류"))?;
    let entry = &manifest[platform];
    let url = entry["url"].as_str().ok_or_else(|| t("다운로드 주소 오류"))?;
    let expected = entry["sha256"].as_str().ok_or_else(|| t("다운로드 검증 정보 오류"))?;
    let parent = data_dir()?;
    fs::create_dir_all(&parent).map_err(|_| t("도구 폴더를 만들지 못했습니다."))?;
    let target = parent.join(format!("platform-tools-{VERSION}"));
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| t("시스템 시각 오류"))?
        .as_nanos();
    // 앱이 준비한 복사본이 정상이면 다시 내려받지 않는다. 폴더만 남았거나 실행이 안 되면
    // 지우지 않고 옆으로 옮겨 새 복사본이 들어갈 자리를 만든다. 사용자 SDK는 대상이 아니다.
    if target.exists() {
        // 유효성은 실행 검사로만 판정한다. 설정 저장 실패는 손상이 아니므로 그대로 보고한다.
        if target.join(executable()).is_file()
            && Adb::inspect(&target.join(executable()), stop).is_ok()
        {
            return select(&target.join(executable()), stop);
        }
        // 취소로 실패한 검사를 손상으로 오인해 정상 복사본을 옮기지 않는다.
        if stop.load(Ordering::Relaxed) {
            return Err(t("준비를 취소했습니다."));
        }
        let aside = parent.join(format!("platform-tools-{VERSION}.broken-{stamp}"));
        fs::rename(&target, &aside).map_err(|_| {
            t("이전에 준비한 도구 폴더를 옮기지 못했습니다. 다른 프로그램이 사용 중인지 확인해 주세요.")
        })?;
    }
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(DOWNLOAD_BUDGET))
        .timeout_connect(Some(Duration::from_secs(5)))
        .timeout_recv_response(Some(Duration::from_secs(10)))
        .max_redirects(0)
        .build();
    let agent: ureq::Agent = config.into();
    let mut response = agent.get(url).call().map_err(|_| t("Google 다운로드에 연결하지 못했습니다. 인터넷 연결을 확인하거나 기존 ADB를 선택해 주세요."))?;
    if !response.status().is_success() {
        return Err(
            t("공식 다운로드 서버가 파일을 제공하지 않았습니다. 잠시 후 다시 시도해 주세요."),
        );
    }
    let mut reader = response
        .body_mut()
        .with_config()
        .limit(ARCHIVE_LIMIT)
        .reader();
    let mut bytes = Vec::new();
    let mut chunk = [0; 65536];
    loop {
        if stop.load(Ordering::Relaxed) {
            return Err(t("준비를 취소했습니다."));
        }
        let n = reader
            .read(&mut chunk)
            .map_err(|_| t("다운로드가 중단되었습니다. 다시 준비를 눌러 주세요."))?;
        if n == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..n]);
    }
    if Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
        != expected
    {
        return Err(t("다운로드 파일의 무결성을 확인하지 못했습니다. 실행하지 않았습니다."));
    }
    let stage = parent.join(format!("download-{}-{stamp}", std::process::id()));
    fs::create_dir(&stage).map_err(|_| t("임시 도구 폴더를 만들지 못했습니다."))?;
    let result = (|| {
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))
            .map_err(|_| t("압축 파일을 열지 못했습니다."))?;
        let names: &[&str] = if cfg!(windows) {
            &[
                "adb.exe",
                "AdbWinApi.dll",
                "AdbWinUsbApi.dll",
                "NOTICE.txt",
                "source.properties",
            ]
        } else {
            &["adb", "NOTICE.txt", "source.properties"]
        };
        for name in names {
            if stop.load(Ordering::Relaxed) {
                return Err(t("준비를 취소했습니다."));
            }
            let mut source = zip
                .by_name(&format!("platform-tools/{name}"))
                .map_err(|_| t("필수 ADB 파일이 없습니다."))?;
            if source.size() > ARCHIVE_LIMIT {
                return Err(t("압축 해제 크기 상한을 초과했습니다."));
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(stage.join(name))
                .map_err(|_| t("도구 파일 생성 실패"))?;
            std::io::copy(&mut source, &mut file).map_err(|_| t("도구 파일 저장 실패"))?;
            file.flush().map_err(|_| t("도구 파일 저장 실패"))?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(stage.join(executable()), fs::Permissions::from_mode(0o755))
                .map_err(|_| t("실행 권한 설정 실패"))?;
        }
        // 검증된 공식 파일만 실행한다. 사용자 SDK는 교체하지 않는다.
        process::run(&stage.join(executable()), &["version"], None, stop, 3)?;
        fs::rename(&stage, &target).map_err(|_| {
            t("도구 폴더를 확정하지 못했습니다. 다른 adb-on 준비가 진행 중인지 확인해 주세요.")
        })?;
        select(&target.join(executable()), stop)
    })();
    if stage.exists() {
        let _ = fs::remove_dir_all(&stage);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::prune_parents;
    use std::fs;

    #[test]
    fn prune_removes_only_empty_app_owned_parents() {
        let base = std::env::temp_dir().join(format!("adb-on-prune-{}", std::process::id()));
        let data = base.join("tinygem").join("adb-on").join("data");
        fs::create_dir_all(&data).unwrap();
        fs::remove_dir_all(&data).unwrap();
        prune_parents(&data);
        assert!(!base.join("tinygem").exists(), "빈 앱 소유 상위 폴더는 지운다");
        assert!(base.exists(), "이름이 다른 상위 폴더는 건드리지 않는다");

        let data = base.join("tinygem").join("adb-on").join("data");
        fs::create_dir_all(&data).unwrap();
        fs::write(base.join("tinygem").join("other.txt"), b"x").unwrap();
        fs::remove_dir_all(&data).unwrap();
        prune_parents(&data);
        assert!(!base.join("tinygem").join("adb-on").exists());
        assert!(base.join("tinygem").join("other.txt").exists(), "비어 있지 않은 폴더는 남긴다");
        fs::remove_dir_all(&base).unwrap();
    }
}
