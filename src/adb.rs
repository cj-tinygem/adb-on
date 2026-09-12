use crate::{
    i18n::{t, tf},
    model::{self, Device, Service},
    process,
};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

pub struct Adb {
    pub path: PathBuf,
    protocol: u32,
    pub version: String,
}
fn local() -> SocketAddr {
    "127.0.0.1:5037".parse().unwrap()
}
fn socket() -> std::io::Result<TcpStream> {
    connect(local())
}
fn connect(address: std::net::SocketAddr) -> std::io::Result<TcpStream> {
    // Windows는 닫힌 로컬 포트의 거부를 약2초 뒤 통지할 수 있다. 너무 짧으면
    // 서버 없음을 시간 초과로 오인해 최초 ADB 시작을 막는다.
    let stream = TcpStream::connect_timeout(&address, Duration::from_secs(3))?;
    stream.set_read_timeout(Some(Duration::from_millis(350)))?;
    stream.set_write_timeout(Some(Duration::from_millis(350)))?;
    Ok(stream)
}
fn request(stream: &mut TcpStream, service: &str) -> Result<(), String> {
    stream
        .write_all(format!("{:04x}{service}", service.len()).as_bytes())
        .map_err(|_| t("ADB 서버 요청 실패"))?;
    let mut status = [0; 4];
    stream
        .read_exact(&mut status)
        .map_err(|_| t("ADB 서버가 응답하지 않습니다."))?;
    if &status != b"OKAY" {
        return Err(t("ADB 서버가 이 요청을 지원하지 않습니다."));
    }
    Ok(())
}
pub fn server_version() -> Result<Option<u32>, String> {
    let mut stream = match socket() {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => return Ok(None),
        Err(_) => return Err(
            t("기존 ADB 서버를 확인하지 못했습니다. 사용 중인 개발 도구의 연결 상태를 확인해 주세요."),
        ),
    };
    request(&mut stream, "host:version")?;
    let mut len = [0; 4];
    stream
        .read_exact(&mut len)
        .map_err(|_| t("ADB 서버 응답 오류"))?;
    let n = usize::from_str_radix(std::str::from_utf8(&len).unwrap_or(""), 16)
        .map_err(|_| t("ADB 서버 응답 오류"))?;
    if n == 0 || n > 16 {
        return Err(t("ADB 서버 응답 오류"));
    }
    let mut value = vec![0; n];
    stream
        .read_exact(&mut value)
        .map_err(|_| t("ADB 서버 응답 오류"))?;
    u32::from_str_radix(std::str::from_utf8(&value).unwrap_or(""), 16)
        .map(Some)
        .map_err(|_| t("ADB 서버 버전 오류"))
}
/// Windows `canonicalize`는 `\\?\C:\…` 확장 경로를 돌려준다. 사용자에게 보이고 설정에 저장하는
/// 경로는 일반 형식으로 두며, UNC(`\\?\UNC\서버\공유`)는 `\\서버\공유`로만 바꾼다.
fn without_verbatim(path: PathBuf) -> PathBuf {
    let Some(text) = path.to_str() else {
        return path;
    };
    let drive = |s: &str| {
        let b = s.as_bytes();
        b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && b[2] == b'\\'
    };
    if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{rest}"))
    } else if let Some(rest) = text.strip_prefix(r"\\?\")
        && drive(rest)
    {
        PathBuf::from(rest)
    } else {
        // `\\?\Volume{GUID}\…`처럼 드라이브 문자가 없는 형식은 접두어를 벗기면 잘못된 경로가 된다.
        path
    }
}
impl Adb {
    pub fn inspect(path: &Path, stop: &Arc<AtomicBool>) -> Result<Self, String> {
        let path = without_verbatim(
            path.canonicalize()
                .map_err(|_| t("ADB 파일을 찾지 못했습니다."))?,
        );
        let expected = if cfg!(windows) { "adb.exe" } else { "adb" };
        if path.file_name().and_then(|s| s.to_str()) != Some(expected) || !path.is_file() {
            return Err(tf("{} 파일을 선택해 주세요.", &[expected]));
        }
        let version = process::run(&path, &["version"], None, stop, 3)?;
        let protocol = version
            .lines()
            .next()
            .and_then(|s| s.strip_prefix("Android Debug Bridge version 1.0."))
            .and_then(|s| s.trim().parse().ok())
            .ok_or_else(|| t("정상적인 Android ADB 파일이 아닙니다."))?;
        Ok(Self {
            path,
            protocol,
            version: version.lines().take(2).collect::<Vec<_>>().join(" / "),
        })
    }
    fn compatible(&self) -> Result<(), String> {
        match server_version()? {
            Some(version) if version != self.protocol => Err(t("다른 버전의 ADB 서버가 사용 중입니다. 개발 도구와 같은 ADB 파일을 선택해 주세요. 기존 연결은 유지했습니다.")),
            _ => Ok(())
        }
    }
    pub fn call(
        &self,
        args: &[&str],
        input: Option<&str>,
        stop: &Arc<AtomicBool>,
    ) -> Result<String, String> {
        self.compatible()?;
        // ADB는 localhost만 로컬 서버로 분류한다. IP 리터럴은 최초 서버 시작을 막는다.
        let mut full = vec!["-H", "localhost", "-P", "5037"];
        full.extend_from_slice(args);
        process::run(&self.path, &full, input, stop, 12)
    }
    pub fn list(&self, stop: &Arc<AtomicBool>) -> Result<Vec<Device>, String> {
        Ok(model::devices(&self.call(
            &["devices", "-l"],
            None,
            stop,
        )?))
    }
    pub fn discover(&self, stop: &Arc<AtomicBool>) -> Result<Vec<Service>, String> {
        Ok(model::services(&self.call(
            &["mdns", "services"],
            None,
            stop,
        )?))
    }
    pub fn pair(&self, address: &str, code: &str, stop: &Arc<AtomicBool>) -> Result<(), String> {
        let address = model::endpoint(address)?.to_string();
        let code = model::pairing_code(code)?;
        let result = self.call(&["pair", &address], Some(&code), stop)?;
        if result.contains("Successfully paired") {
            Ok(())
        } else {
            Err(t("페어링되지 않았습니다. 새 6자리 코드와 페어링 주소로 다시 시도해 주세요."))
        }
    }
    pub fn connect(&self, address: &str, stop: &Arc<AtomicBool>) -> Result<(), String> {
        let address = model::endpoint(address)?.to_string();
        let result = self.call(&["connect", &address], None, stop)?;
        if result.starts_with("connected to ") || result.starts_with("already connected to ") {
            Ok(())
        } else {
            Err(t("연결되지 않았습니다. 무선 디버깅 첫 화면의 IP 주소와 포트를 확인해 주세요. 페어링 포트와 다릅니다."))
        }
    }
    pub fn disconnect(&self, serial: &str, stop: &Arc<AtomicBool>) -> Result<(), String> {
        // 화면에 실제로 존재하는 네트워크 기기 하나만 해제한다. USB와 전체 해제는 제공하지 않는다.
        let list = self.list(stop)?;
        if !list.iter().any(|d| d.serial == serial) || !model::wireless(serial) {
            return Err(t("선택한 무선 기기가 현재 연결되어 있지 않습니다."));
        }
        self.call(&["disconnect", serial], None, stop).map(|_| ())
    }
    pub fn track(&self) -> Result<Tracker, String> {
        self.compatible()?;
        let mut stream = socket().map_err(|_| t("ADB 상태 알림 연결 실패"))?;
        request(&mut stream, "host:track-devices-l")?;
        Ok(Tracker {
            stream,
            pending: Vec::new(),
        })
    }
}
/// ADB 스마트 소켓의 상태 알림만 읽는다. 기기 전송·인증 프로토콜은 ADB가 소유한다.
pub struct Tracker {
    stream: TcpStream,
    pending: Vec<u8>,
}
impl Tracker {
    /// 도착한 알림 프레임을 읽고 가장 최근 기기 목록을 돌려준다. 새 프레임이 없으면 `None`이다.
    pub fn poll(&mut self) -> Result<Option<Vec<Device>>, String> {
        let mut bytes = [0; 4096];
        match self.stream.read(&mut bytes) {
            Ok(0) => return Err(t("ADB 서버 연결이 끊어졌습니다.")),
            Ok(n) => self.pending.extend_from_slice(&bytes[..n]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => return Err(t("ADB 서버 연결이 끊어졌습니다.")),
        }
        if self.pending.len() > 128 * 1024 {
            return Err(t("ADB 상태 응답 크기 초과"));
        }
        let mut latest = None;
        while self.pending.len() >= 4 {
            let n =
                usize::from_str_radix(std::str::from_utf8(&self.pending[..4]).unwrap_or(""), 16)
                    .map_err(|_| t("ADB 상태 응답 오류"))?;
            if self.pending.len() < 4 + n {
                break;
            }
            latest = Some(model::devices(&String::from_utf8_lossy(
                &self.pending[4..4 + n],
            )));
            self.pending.drain(..4 + n);
        }
        Ok(latest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_local_server_is_refused_not_timed_out() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        assert_eq!(
            connect(address).unwrap_err().kind(),
            std::io::ErrorKind::ConnectionRefused
        );
    }
    #[test]
    fn device_events_survive_partial_headers_and_multiple_frames() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        client
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        let (mut server, _) = listener.accept().unwrap();
        let mut tracker = Tracker {
            stream: client,
            pending: Vec::new(),
        };
        server.write_all(b"000").unwrap();
        assert!(tracker.poll().unwrap().is_none());
        let body = "serial\tunauthorized\n";
        let frame = format!("{:04x}{body}", body.len());
        server
            .write_all(format!("0{frame}0000").as_bytes())
            .unwrap();
        assert_eq!(tracker.poll().unwrap(), Some(vec![]));
        server.write_all(frame.as_bytes()).unwrap();
        assert_eq!(tracker.poll().unwrap().unwrap()[0].state, "unauthorized");
    }
    #[test]
    fn verbatim_prefix_is_removed_only_for_windows_forms() {
        let plain = |s: &str| {
            without_verbatim(PathBuf::from(s))
                .to_str()
                .unwrap()
                .to_string()
        };
        assert_eq!(plain(r"\\?\C:\Sdk\adb.exe"), r"C:\Sdk\adb.exe");
        assert_eq!(plain(r"\\?\UNC\nas\sdk\adb.exe"), r"\\nas\sdk\adb.exe");
        assert_eq!(plain(r"C:\Sdk\adb.exe"), r"C:\Sdk\adb.exe");
        assert_eq!(plain("/opt/sdk/adb"), "/opt/sdk/adb");
        let volume = r"\\?\Volume{1b2c}\sdk\adb.exe";
        assert_eq!(plain(volume), volume);
    }
}
