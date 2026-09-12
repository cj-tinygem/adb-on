#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
use std::sync::{Arc, atomic::AtomicBool};

fn diagnostics() -> bool {
    if std::env::args().nth(1).as_deref() != Some("--self-test") {
        return false;
    }
    let stop = Arc::new(AtomicBool::new(false));
    let found = adb_on::setup::locate(&stop);
    let (path, version, error) = match found {
        Ok(Some(adb)) => (
            Some(adb.path.display().to_string()),
            Some(adb.version),
            None,
        ),
        Ok(None) => (None, None, None),
        Err(e) => (None, None, Some(e)),
    };
    let server = adb_on::adb::server_version();
    let failed = error.is_some() || server.is_err();
    println!(
        "{}",
        serde_json::json!({"product":"adb-on","version":env!("CARGO_PKG_VERSION"),"status":if failed {"error"} else {"ok"},"gui":"slint","webview":false,"adb_path":path,"adb_version":version,"error":error,"server_protocol":server.as_ref().ok().copied().flatten(),"server_error":server.err(),"mutates_devices":false})
    );
    if failed {
        std::process::exit(1);
    }
    true
}
#[cfg(feature = "gui")]
slint::include_modules!();
/// Windows에서는 한 번에 한 창만 둔다. 두 번째 실행은 이미 열린 창을 앞으로 가져오고 끝난다.
/// 같은 설정 파일과 도구 폴더를 두 인스턴스가 동시에 바꾸는 일을 막는다.
#[cfg(all(windows, feature = "gui"))]
fn focus_existing_instance() -> bool {
    use windows_sys::Win32::{
        Foundation::{ERROR_ALREADY_EXISTS, GetLastError},
        System::Threading::CreateMutexW,
        UI::WindowsAndMessaging::{FindWindowW, IsIconic, SW_RESTORE, SetForegroundWindow, ShowWindow},
    };
    let name: Vec<u16> = "Local\\adb-on.single-instance\0".encode_utf16().collect();
    // 핸들은 프로세스가 끝날 때까지 쥐고 있어야 하므로 닫지 않는다.
    let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
    if handle.is_null() || unsafe { GetLastError() } != ERROR_ALREADY_EXISTS {
        return false;
    }
    let title: Vec<u16> = "adb-on\0".encode_utf16().collect();
    let window = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
    if !window.is_null() {
        unsafe {
            if IsIconic(window) != 0 {
                ShowWindow(window, SW_RESTORE);
            }
            SetForegroundWindow(window);
        }
    }
    true
}
#[cfg(all(not(windows), feature = "gui"))]
fn focus_existing_instance() -> bool {
    false
}
/// 번들 번역에서 고를 이름. 원문(한국어)은 빈 문자열이 원문을 뜻한다.
#[cfg(feature = "gui")]
fn bundled_language(code: &str) -> &str {
    if code == "ko" { "" } else { code }
}
#[cfg(feature = "gui")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if diagnostics() {
        return Ok(());
    }
    use adb_on::{
        i18n::{self, t},
        model,
        worker::{Action, View},
    };
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::{rc::Rc, sync::mpsc};
    if focus_existing_instance() {
        return Ok(());
    }
    let ui = AppWindow::new()?;
    // 언어: 저장된 선택이 있으면 그것, 없으면 이 PC의 언어. 첫 컴포넌트를 만든 뒤에 번들 번역을 고른다.
    let saved = adb_on::setup::saved_language();
    let code = match saved.as_deref() {
        Some(code) if i18n::set(code) => code.to_string(),
        _ => {
            let detected = i18n::detect(sys_locale::get_locale().as_deref());
            i18n::set(detected);
            detected.to_string()
        }
    };
    // 한국어는 번들의 원문이라 번역 파일이 없다. 원문으로 돌아가려면 빈 언어를 골라야 한다.
    let _ = slint::select_bundled_translation(bundled_language(&code));
    ui.set_languages(ModelRc::from(Rc::new(VecModel::from(
        i18n::LANGUAGES.iter().map(|(_, name)| SharedString::from(*name)).collect::<Vec<_>>(),
    ))));
    ui.set_language_fonts(ModelRc::from(Rc::new(VecModel::from(
        i18n::LANGUAGES.iter().map(|(code, _)| SharedString::from(i18n::font_family(code))).collect::<Vec<_>>(),
    ))));
    ui.set_language_index(i18n::index() as i32);
    ui.set_font_family(i18n::font_family(&code).into());
    ui.set_version(env!("CARGO_PKG_VERSION").into());
    ui.set_data_path(adb_on::setup::data_dir_display().into());
    let (sender, receiver) = mpsc::sync_channel::<Action>(8);
    let stop = Arc::new(AtomicBool::new(false));
    let weak = ui.as_weak();
    let stopped = stop.clone();
    let (done_tx, done_rx) = mpsc::channel::<()>();
    let worker = std::thread::spawn(move || {
        adb_on::worker::run(receiver, stopped, move |view: View| {
            let _ = weak.upgrade_in_event_loop(move |ui| {
                ui.set_ready(view.ready);
                // 일반 상태 알림은 사용자가 방금 건 요청의 잠금을 풀지 않는다.
                if view.finished {
                    ui.set_busy(false);
                } else if view.busy {
                    ui.set_busy(true);
                }
                ui.set_status(view.title.into());
                ui.set_lead(view.lead.into());
                ui.set_connection(view.connection);
                ui.set_guide_hint(view.guide_hint);
                ui.set_link_step(view.link_step);
                ui.set_adb_help(view.adb_help);
                ui.set_searched(view.searched);
                ui.set_detail_pre(view.detail_pre.into());
                ui.set_detail_link(view.detail_link.into());
                ui.set_detail_post(view.detail_post.into());
                // 접힌 한 줄에는 첫 문단만 보여 준다. 줄바꿈이 들어가면 말줄임이 두 줄로 그려진다.
                ui.set_detail_summary(view.detail.split('\n').next().unwrap_or_default().into());
                ui.set_detail(view.detail.into());
                ui.set_notice(view.notice.into());
                ui.set_adb_path(view.path.into());
                ui.set_adb_source(view.source.into());
                ui.set_adb_kind(view.source_kind);
                // 찾은 휴대폰 버튼: 기기 목록에 이미 있는 폰이면 모델명을 앞에 붙이고, 무선으로 연결돼 있으면 잠근다.
                let row = |s: &model::Service| {
                    let known = view.devices.iter().find(|d| model::same_phone(s, d));
                    ServiceRow {
                        address: s.address.to_string().into(),
                        label: match known {
                            Some(d) => format!("{} · {}", d.name, s.address).into(),
                            None => s.address.to_string().into(),
                        },
                        connected: known.is_some_and(|d| d.state == "device" && model::wireless(&d.serial)),
                    }
                };
                let (pairing, found): (Vec<_>, Vec<_>) = view.services.iter().partition(|s| s.pairing);
                if pairing.len() == 1 && ui.get_pairing_address().is_empty() {
                    ui.set_pairing_address(pairing[0].address.to_string().into());
                }
                ui.set_found_phones(ModelRc::from(Rc::new(VecModel::from(found.iter().map(|s| row(s)).collect::<Vec<_>>()))));
                ui.set_pairing_phones(ModelRc::from(Rc::new(VecModel::from(pairing.iter().map(|s| row(s)).collect::<Vec<_>>()))));
                ui.set_devices(ModelRc::from(Rc::new(VecModel::from(
                    view.devices
                        .into_iter()
                        .map(|d| DeviceRow {
                            wireless: model::wireless(&d.serial),
                            connected: d.state == "device",
                            serial: d.serial.into(),
                            name: d.name.into(),
                            state: t(match d.state.as_str() {
                                "device" => "연결됨",
                                "unauthorized" => "휴대폰에서 승인 대기",
                                "offline" => "응답 없음",
                                _ => "일반 연결 모드 아님",
                            })
                            .into(),
                        })
                        .collect::<Vec<_>>(),
                ))));
            });
        });
        let _ = done_tx.send(());
    });
    // 모든 콜백은 같은 유한 큐에 요청만 넣는다. 중복 조작은 UI에서 즉시 잠근다.
    let send = Rc::new({
        let ui = ui.as_weak();
        move |action| {
            if let Some(ui) = ui.upgrade() {
                if ui.get_busy() {
                    return;
                }
                match sender.try_send(action) {
                    Ok(()) => ui.set_busy(true),
                    Err(_) => {
                        ui.set_notice(t("작업을 시작하지 못했어요. 앱을 다시 열어 주세요.").into())
                    }
                }
            }
        }
    });
    {
        let s = send.clone();
        ui.on_refresh(move || s(Action::Refresh));
    }
    {
        let s = send.clone();
        ui.on_discover(move || s(Action::Discover));
    }
    {
        let s = send.clone();
        ui.on_pair(move |a, c| s(Action::Pair(a.into(), c.into())));
    }
    {
        let s = send.clone();
        ui.on_connect(move |a| s(Action::Connect(a.into())));
    }
    {
        let s = send.clone();
        ui.on_disconnect(move |a| s(Action::Disconnect(a.into())));
    }
    {
        let s = send.clone();
        ui.on_install(move |c| s(Action::Install(c)));
    }
    {
        let s = send.clone();
        ui.on_remove_data(move || s(Action::RemoveData));
    }
    {
        let s = send.clone();
        let w = ui.as_weak();
        ui.on_set_language(move |index| {
            let Some((code, _)) = i18n::LANGUAGES.get(index.max(0) as usize) else { return };
            if i18n::set(code) {
                let _ = slint::select_bundled_translation(bundled_language(code));
                // 페이지를 오가면 설정 화면이 다시 만들어지므로 선택값과 글꼴을 창 속성에도 남긴다.
                if let Some(ui) = w.upgrade() {
                    ui.set_language_index(index);
                    ui.set_font_family(i18n::font_family(code).into());
                }
                s(Action::Language(code.to_string()));
            }
        });
    }
    {
        let s = send.clone();
        ui.on_choose_adb(move || {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(&t("ADB 실행 파일 선택"))
                .pick_file()
            {
                s(Action::Select(path));
            }
        });
    }
    ui.on_open_link(|id| {
        let url = match id {
            0 => adb_on::setup::TERMS,
            1 => "https://developer.android.com/studio/run/oem-usb",
            _ => "https://slint.dev",
        };
        let _ = open::that_detached(url);
    });
    let result = ui.run();
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    drop(send);
    // 유한 자식은 취소 표식 뒤 수십 ms 안에 회수된다. 멈춘 다운로드 읽기는 자식이 없으므로
    // 예산까지 기다리지 않고 프로세스 종료로 끝낸다.
    if done_rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .is_ok()
    {
        let _ = worker.join();
    }
    result?;
    Ok(())
}
#[cfg(not(feature = "gui"))]
fn main() {
    if !diagnostics() {
        eprintln!("이 빌드는 코어 검증용입니다. GUI 빌드를 실행해 주세요.");
    }
}
