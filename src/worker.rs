use crate::{
    i18n::{t, tf},
    adb::{Adb, Tracker},
    model::{Device, Service},
    setup,
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, RecvTimeoutError},
    },
    time::{Duration, Instant},
};

pub enum Action {
    Refresh,
    Discover,
    Pair(String, String),
    Connect(String),
    Disconnect(String),
    Install(bool),
    Select(PathBuf),
    RemoveData,
    Language(String),
}
#[derive(Clone, Default)]
pub struct View {
    pub ready: bool,
    pub busy: bool,
    pub finished: bool,
    pub title: String,
    /// 앱바 점 색의 근거. 0 = 도구 없음(회색), 1 = 폰 없음(빨강), 2 = 조치 필요(노랑), 3 = 연결됨(초록).
    pub connection: i32,
    /// 홈 카드에서 바로 열어 줄 USB 안내 단계. 없으면 -1.
    pub guide_hint: i32,
    /// 설명 안 글자 링크가 열어 줄 USB 안내 단계(개발자 옵션 여는 법 등). 없으면 -1.
    pub link_step: i32,
    /// 연결된 뒤 "ADB로 무엇을 할 수 있나요?" 버튼을 보일지.
    pub adb_help: bool,
    /// 링크가 있는 설명은 세 조각으로 나눠 준다: 링크 앞 글, 링크 글자, 링크 뒤 글. 링크가 없으면 모두 빈 문자열.
    pub detail_pre: String,
    pub detail_link: String,
    pub detail_post: String,
    /// 상세 설명 앞에 굵게 보여 줄 한 문장. 비어 있으면 상세만 보인다.
    pub lead: String,
    pub detail: String,
    pub notice: String,
    pub path: String,
    pub source: String,
    pub source_kind: i32,
    pub devices: Vec<Device>,
    pub services: Vec<Service>,
    /// 무선 탐색을 한 번이라도 끝냈는지. 결과가 비었을 때 "찾지 못했어요"를 화면에 보이기 위한 값.
    pub searched: bool,
}
fn describe(view: &mut View) {
    view.lead.clear();
    view.guide_hint = -1;
    view.link_step = -1;
    view.adb_help = false;
    view.detail_pre.clear();
    view.detail_link.clear();
    view.detail_post.clear();
    view.connection = if !view.ready {
        0
    } else if view.devices.iter().any(|d| d.state == "device") {
        3
    } else if view.devices.is_empty() {
        1
    } else {
        2
    };
    if !view.ready {
        view.title = t("처음 한 번만 준비해 주세요");
        view.detail = t("휴대폰과 연결하려면 Google 도구(ADB)가 필요해요. 기존 도구를 찾거나 여기서 준비할 수 있어요.");
    } else if view.devices.iter().any(|d| d.state == "device") {
        // 기기 이름은 아래 기기 줄에 이미 보이므로 여기서는 되풀이하지 않는다.
        view.title = t("연결됐어요");
        view.detail = t("이 PC의 개발 도구에서 휴대폰을 선택하세요.");
        view.adb_help = true;
    } else if view.devices.iter().any(|d| d.state == "unauthorized") {
        view.title = t("휴대폰에서 ‘허용’을 눌러 주세요");
        view.lead = t("휴대폰의 USB 디버깅은 켜져 있어요. 이 PC를 허용하는 일만 남았어요.");
        view.detail_pre = t("휴대폰 잠금을 풀면 ‘USB 디버깅을 허용하시겠습니까?’ 창이 떠요. ‘이 컴퓨터에서 항상 허용’을 켜고 허용을 누르세요. 상단의 ‘USB로 파일 전송’ 알림은 이 창이 아니에요.\n\n창이 안 보이면 잠금을 푼 채 케이블을 뽑았다 다시 꽂아 보세요. 알림을 내렸을 때 ‘USB 디버깅 연결됨’이 보이면 디버깅은 켜진 거예요.\n\n그래도 없으면 ");
        view.detail_link = t("개발자 옵션");
        view.detail_post = t("에서 ‘USB 디버깅 권한 취소’를 누르고, ‘USB 디버깅’을 껐다가 다시 켠 뒤 케이블을 다시 꽂으세요. 개발자 옵션은 휴대폰 설정 앱의 맨 아래에 있어요. 없으면 위 링크의 1단계대로 만드세요.");
        view.detail = format!("{}{}{}", view.detail_pre, view.detail_link, view.detail_post);
        view.guide_hint = 2;
        view.link_step = 0;
    } else if view.devices.iter().any(|d| d.state == "offline") {
        view.title = t("휴대폰이 응답을 기다리고 있어요");
        view.detail = t("잠금을 풀고 케이블을 다시 연결해 주세요. 무선이면 휴대폰의 무선 디버깅이 켜져 있는지 확인해 주세요.");
        view.guide_hint = 3;
    } else if !view.devices.is_empty() {
        view.title = t("휴대폰이 일반 실행 상태가 아니에요");
        view.detail = t("복구·부트로더 모드에서는 앱 개발 연결을 사용할 수 없어요. 휴대폰을 정상 모드로 켜 주세요.");
    } else {
        view.title = t("휴대폰을 연결해 주세요");
        view.lead = t("USB 케이블을 꽂으면 자동으로 확인해요.");
        view.detail = t("케이블 없이 연결하려면 아래 ‘무선 연결’을 눌러 주세요.");
    }
}
fn ready(adb: &Adb, view: &mut View, stop: &Arc<AtomicBool>) -> Result<Tracker, String> {
    view.ready = true;
    view.path = setup::display_path(&adb.path);
    (view.source_kind, view.source) = setup::describe_source(&adb.path);
    view.devices = adb.list(stop)?;
    describe(view);
    adb.track()
}
pub fn run(rx: Receiver<Action>, stop: Arc<AtomicBool>, publish: impl Fn(View)) {
    let mut view = View::default();
    let mut adb = match setup::locate(&stop) {
        Ok(adb) => adb,
        Err(error) => {
            view.notice = error;
            None
        }
    };
    let mut tracker = None;
    if let Some(ref tool) = adb {
        match ready(tool, &mut view, &stop) {
            Ok(t) => tracker = Some(t),
            Err(e) => view.notice = e,
        }
    }
    describe(&mut view);
    publish(view.clone());
    let mut retry = Instant::now();
    let mut discovery_until = Instant::now();
    let mut discovery_at = Instant::now();
    while !stop.load(Ordering::Relaxed) {
        let action = if tracker.is_some() {
            rx.try_recv().ok()
        } else {
            match rx.recv_timeout(Duration::from_millis(350)) {
                Ok(a) => Some(a),
                Err(RecvTimeoutError::Disconnected) => break,
                Err(_) => None,
            }
        };
        if let Some(action) = action {
            if matches!(
                &action,
                Action::Refresh | Action::Install(_) | Action::Select(_) | Action::RemoveData | Action::Language(_)
            ) {
                discovery_until = Instant::now();
            }
            if matches!(
                &action,
                Action::Refresh
                    | Action::Install(_)
                    | Action::Select(_)
                    | Action::RemoveData
                    | Action::Language(_)
                    | Action::Discover
            ) {
                view.services.clear();
            }
            view.busy = true;
            view.notice.clear();
            publish(view.clone());
            let result = match action {
                Action::Install(consented) => setup::install(consented, &stop).map(|tool| {
                    adb = Some(tool);
                    tracker = None;
                    String::new()
                }),
                Action::Select(path) => setup::select(&path, &stop).map(|tool| {
                    adb = Some(tool);
                    tracker = None;
                    String::new()
                }),
                Action::Refresh => setup::locate(&stop).map(|found| {
                    adb = found;
                    tracker = None;
                    String::new()
                }),
                Action::Language(code) => setup::save_language(&code).map(|_| {
                    // 저장 뒤에는 상태 문장을 새 언어로 다시 만든다. 안내 문구는 띄우지 않는다.
                    String::new()
                }),
                Action::RemoveData => setup::remove_data(&stop).map(|_| {
                    // 저장한 ADB 위치가 사라졌으므로 SDK·PATH의 ADB를 다시 찾는다. 결과는 도구 설정 화면의 출처 표시가 보여 준다.
                    adb = setup::locate(&stop).unwrap_or(None);
                    tracker = None;
                    String::new()
                }),
                action => match adb.as_ref() {
                    None => Err(t("먼저 ADB를 준비해 주세요.")),
                    Some(tool) => match action {
                        Action::Discover => {
                            discovery_until = Instant::now() + Duration::from_secs(24);
                            discovery_at = Instant::now();
                            // 결과는 무선 연결 화면이 직접 보여 준다(찾은 휴대폰 카드 또는 "찾지 못했어요" 한 줄). 알림은 띄우지 않는다.
                            tool.discover(&stop).map(|services| {
                                view.services = services;
                                view.searched = true;
                                String::new()
                            })
                        }
                        Action::Pair(address, mut code) => {
                            let paired = tool.pair(&address, &code, &stop);
                            code.clear();
                            paired.map(|_| {
                                // 페어링은 이미 성공했다. 뒤따르는 확인이 실패해도 성공을 오류로 바꾸지 않는다.
                                // mDNS 자동 연결을 먼저 존중하며 주소를 저장해 낡은 포트로 재시도하지 않는다.
                                let followup = (|| -> Result<(), String> {
                                    let ip = crate::model::endpoint(&address)?.ip();
                                    view.devices = tool.list(&stop)?;
                                    view.services = tool.discover(&stop).unwrap_or_default();
                                    let candidates: Vec<_> = view
                                        .services
                                        .iter()
                                        .filter(|s| !s.pairing && s.address.ip() == ip)
                                        .collect();
                                    if candidates.len() == 1 {
                                        let _ = tool.connect(&candidates[0].address.to_string(), &stop);
                                        view.devices = tool.list(&stop)?;
                                    }
                                    Ok(())
                                })();
                                let mut notice = t("페어링됐어요. 연결 상태를 확인 중이에요. 연결이 안 되면 휴대폰에서 한 화면 뒤로 가서 ‘IP 주소 및 포트’를 아래 연결 칸에 입력해 주세요.");
                                if let Err(e) = followup {
                                    notice = tf("페어링됐어요. 연결 상태 확인은 실패했어요: {} 다시 확인을 눌러 주세요.", &[&e]);
                                }
                                notice
                            })
                        }
                        Action::Connect(address) => tool
                            .connect(&address, &stop)
                            .map(|_| String::new()),
                        Action::Disconnect(serial) => tool.disconnect(&serial, &stop).map(|_| String::new()),
                        Action::Refresh
                        | Action::Install(_)
                        | Action::Select(_)
                        | Action::RemoveData
                        | Action::Language(_) => {
                            unreachable!("위 분기에서 처리한 액션")
                        }
                    },
                },
            };
            view.notice = result.unwrap_or_else(|e| e);
            view.busy = false;
            if tracker.is_none() {
                view.ready = adb.is_some();
                if adb.is_none() {
                    view.path.clear();
                    view.source.clear();
                    view.source_kind = 0;
                }
                if let Some(ref tool) = adb {
                    match ready(tool, &mut view, &stop) {
                        Ok(t) => tracker = Some(t),
                        Err(e) => {
                            // 방금 끝난 작업의 결과는 지우지 않고 상태 확인 실패를 뒤에 붙인다.
                            view.devices.clear();
                            view.notice = if view.notice.is_empty() {
                                e
                            } else {
                                format!("{} {e}", view.notice)
                            };
                        }
                    }
                } else {
                    view.devices.clear();
                }
                retry = Instant::now();
            }
            describe(&mut view);
            view.finished = true;
            publish(view.clone());
            view.finished = false;
        }
        if Instant::now() < discovery_until && discovery_at.elapsed() >= Duration::from_secs(3) {
            if let Some(ref tool) = adb {
                let services = tool.discover(&stop).unwrap_or_default();
                if services != view.services {
                    view.services = services;
                    publish(view.clone());
                }
            }
            discovery_at = Instant::now();
        }
        if let Some(ref mut active) = tracker {
            match active.poll() {
                Ok(Some(devices)) => {
                    view.devices = devices;
                    describe(&mut view);
                    publish(view.clone());
                }
                Ok(None) => {}
                Err(error) => {
                    tracker = None;
                    retry = Instant::now();
                    view.devices.clear();
                    describe(&mut view);
                    view.notice =
                        tf("{} 다시 확인을 누르면 서버 상태를 확인하고 연결을 준비해요.", &[&error]);
                    publish(view.clone());
                }
            }
        } else if retry.elapsed() >= Duration::from_secs(5) {
            if let Some(ref tool) = adb {
                // 조회로만 재접속한다. 사용자 서버를 자동으로 재시작하지 않는다.
                if let Ok(tracked) = tool.track() {
                    tracker = Some(tracked);
                    view.notice.clear();
                    publish(view.clone());
                }
            }
            retry = Instant::now();
        }
    }
}
