use crate::{
    i18n::{t, tf},
    adb::{Adb, Tracker},
    model::{Device, Service},
    setup,
};
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, SocketAddr},
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
    WirelessPage(bool),
    ManualAddresses(bool),
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
    pub discovery_notice: String,
    pub server_notice: String,
    pub paired_ip: String,
}

// Keep brief mDNS gaps from erasing a candidate while the user opens the pairing dialog.
// This cache is session-only; fresh advertisements replace a phone's previous port.
#[derive(Default)]
struct Discovery {
    services: Vec<(Service, Instant)>,
    last_connected: HashMap<IpAddr, SocketAddr>,
}
impl Discovery {
    fn connection_address(&self, ip: IpAddr) -> Option<SocketAddr> {
        let candidates: Vec<_> = self.services.iter()
            .filter(|(s, _)| !s.pairing && s.address.ip() == ip)
            .map(|(s, _)| s.address).collect();
        match candidates.as_slice() {
            [address] => Some(*address),
            [] => self.last_connected.get(&ip).copied(),
            _ => None,
        }
    }
    fn update(&mut self, services: Vec<Service>, now: Instant) {
        for mut service in services {
            if let Some((old, _)) = self.services.iter().find(|(old, _)| old.address == service.address) {
                service.label = old.label.clone();
            }
            self.services.retain(|(old, _)| !(old.pairing == service.pairing
                && (old.address.ip() == service.address.ip()
                    || (!old.name.is_empty() && old.name == service.name))));
            self.services.push((service, now));
        }
        self.services.retain(|(service, seen)| now.duration_since(*seen)
            < Duration::from_secs(if service.pairing { 12 } else { 120 }));
    }
    fn show(&mut self, view: &mut View) {
        // An established connection remains a candidate even when multicast discovery fails.
        for device in view.devices.iter().filter(|d| d.state == "device") {
            if let Ok(address) = crate::model::endpoint(&device.serial) {
                self.last_connected.insert(address.ip(), address);
                if !self.services.iter().any(|(s, _)| !s.pairing && s.address == address) {
                    self.services.push((Service { name: String::new(), address, pairing: false, label: device.label() }, Instant::now()));
                }
            }
        }
        for (service, seen) in &mut self.services {
            if let Some(device) = view.devices.iter().find(|d| d.state == "device" && crate::model::same_phone(service, d)) {
                service.label = device.label();
                *seen = Instant::now();
                if !service.pairing { self.last_connected.insert(service.address.ip(), service.address); }
            }
        }
        view.services = self.services.iter().map(|(s, _)| s.clone()).collect();
    }
}

fn set_devices(tool: &Adb, view: &mut View, mut devices: Vec<Device>,
    names: &mut HashMap<String, String>, stop: &Arc<AtomicBool>) {
    names.retain(|serial, _| devices.iter().any(|d| &d.serial == serial && d.state == "device"));
    for device in &mut devices {
        if device.state == "device" {
            device.device_name = names.entry(device.serial.clone())
                .or_insert_with(|| tool.device_name(&device.serial, stop).unwrap_or_default()).clone();
        }
    }
    view.devices = devices;
}

struct PendingPair {
    ip: IpAddr,
    started: Instant,
    attempted: HashSet<SocketAddr>,
}

fn finish_pair(tool: &Adb, view: &mut View, pending: &mut Option<PendingPair>,
    names: &mut HashMap<String, String>, discovery: &Discovery, stop: &Arc<AtomicBool>) {
    let Ok(ip) = view.paired_ip.parse::<IpAddr>() else { return };
    let connected = |view: &View| view.devices.iter().any(|d| d.state == "device"
        && (crate::model::endpoint(&d.serial).is_ok_and(|a| a.ip() == ip)
            || view.services.iter().any(|s| !s.pairing && s.address.ip() == ip
                && crate::model::same_phone(s, d))));
    if connected(view) {
        view.paired_ip.clear();
        *pending = None;
        return;
    }
    let Some(pair) = pending.as_mut() else { return };
    let address = discovery.connection_address(pair.ip);
    if let Some(address) = address && pair.attempted.insert(address) {
        // Pairing has succeeded independently of the connection attempt.
        if tool.connect(&address.to_string(), stop).is_ok()
            && let Ok(devices) = tool.list(stop) {
            set_devices(tool, view, devices, names, stop);
            if connected(view) {
                view.paired_ip.clear();
                *pending = None;
                return;
            }
        }
    }
    if pair.started.elapsed() >= Duration::from_secs(30) {
        *pending = None;
    }
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
    } else if !view.server_notice.is_empty() {
        2
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
fn ready(adb: &Adb, view: &mut View, names: &mut HashMap<String, String>, stop: &Arc<AtomicBool>) -> Result<Tracker, String> {
    view.ready = true;
    view.path = setup::display_path(&adb.path);
    (view.source_kind, view.source) = setup::describe_source(&adb.path);
    set_devices(adb, view, adb.list(stop)?, names, stop);
    describe(view);
    adb.track()
}
pub fn run(rx: Receiver<Action>, stop: Arc<AtomicBool>, publish: impl Fn(View)) {
    let mut view = View::default();
    let mut names = HashMap::new();
    let mut discovery = Discovery::default();
    let mut pending_pair = None;
    let mut adb = match setup::locate(&stop) {
        Ok(adb) => adb,
        Err(error) => {
            view.notice = error;
            None
        }
    };
    let mut tracker = None;
    if let Some(ref tool) = adb {
        match ready(tool, &mut view, &mut names, &stop) {
            Ok(t) => tracker = Some(t),
            Err(e) => view.notice = e,
        }
    }
    describe(&mut view);
    publish(view.clone());
    let mut retry = Instant::now();
    let mut wireless_page = false;
    let mut manual_addresses = false;
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
            if let Action::WirelessPage(active) = action {
                wireless_page = active;
                if active { discovery_at = Instant::now() - Duration::from_secs(3); }
                continue;
            }
            if let Action::ManualAddresses(manual) = action {
                manual_addresses = manual;
                pending_pair = None;
                if !manual {
                    discovery_at = Instant::now() - Duration::from_secs(3);
                    if let Ok(ip) = view.paired_ip.parse() {
                        pending_pair = Some(PendingPair { ip, started: Instant::now(), attempted: HashSet::new() });
                    }
                }
                continue;
            }
            if manual_addresses && matches!(action, Action::Discover) { continue; }
            if matches!(
                &action,
                Action::Refresh | Action::Install(_) | Action::Select(_) | Action::RemoveData | Action::Language(_)
            ) {
                names.clear();
            }
            if matches!(
                &action,
                Action::Refresh
                    | Action::Install(_)
                    | Action::Select(_)
                    | Action::RemoveData
                    | Action::Language(_)
            ) {
                view.services.clear();
                discovery = Discovery::default();
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
                            discovery_at = Instant::now();
                            // 결과는 무선 연결 화면이 직접 보여 준다(찾은 휴대폰 카드 또는 "찾지 못했어요" 한 줄). 알림은 띄우지 않는다.
                            tool.discover(&stop).map(|services| {
                                discovery.update(services, Instant::now());
                                discovery.show(&mut view);
                                view.discovery_notice.clear();
                                view.searched = true;
                                String::new()
                            })
                        }
                        Action::Pair(address, mut code) => {
                            let paired = tool.pair(&address, &code, &stop);
                            code.clear();
                            paired.map(|_| {
                                if let Ok(endpoint) = crate::model::endpoint(&address) {
                                    view.paired_ip = endpoint.ip().to_string();
                                    pending_pair = Some(PendingPair { ip: endpoint.ip(), started: Instant::now(), attempted: HashSet::new() });
                                }
                                if let Ok(devices) = tool.list(&stop) {
                                    set_devices(tool, &mut view, devices, &mut names, &stop);
                                }
                                if !manual_addresses && let Ok(services) = tool.discover(&stop) {
                                    discovery.update(services, Instant::now());
                                }
                                discovery.show(&mut view);
                                finish_pair(tool, &mut view, &mut pending_pair, &mut names, &discovery, &stop);
                                // Manual mode still completes this requested pairing using a known
                                // connection address, but never starts discovery or background retries.
                                if manual_addresses { pending_pair = None; }
                                discovery_at = Instant::now() - Duration::from_secs(3);
                                String::new()
                            })
                        }
                        Action::Connect(address) => tool.connect(&address, &stop).map(|_| {
                            if crate::model::endpoint(&address).is_ok_and(|a| a.ip().to_string() == view.paired_ip) {
                                view.paired_ip.clear();
                                pending_pair = None;
                            }
                            String::new()
                        }),
                        Action::Disconnect(serial) => tool.disconnect(&serial, &stop).map(|_| String::new()),
                        Action::Refresh
                        | Action::Install(_)
                        | Action::Select(_)
                        | Action::RemoveData
                        | Action::Language(_) => {
                            unreachable!("위 분기에서 처리한 액션")
                        }
                        Action::WirelessPage(_) | Action::ManualAddresses(_) => unreachable!("wireless mode handled before commands"),
                    },
                },
            };
            view.notice = result.unwrap_or_else(|e| e);
            view.busy = false;
            if let Some(ref tool) = adb
                && let Ok(devices) = tool.list(&stop) {
                set_devices(tool, &mut view, devices, &mut names, &stop);
            }
            if tracker.is_none() {
                view.ready = adb.is_some();
                if adb.is_none() {
                    view.path.clear();
                    view.source.clear();
                    view.source_kind = 0;
                }
                if let Some(ref tool) = adb {
                    match ready(tool, &mut view, &mut names, &stop) {
                        Ok(t) => { tracker = Some(t); view.server_notice.clear(); }
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
            discovery.show(&mut view);
            view.finished = true;
            publish(view.clone());
            view.finished = false;
        }
        if !manual_addresses && (wireless_page || pending_pair.is_some()) && discovery_at.elapsed() >= Duration::from_secs(3) {
            if let Some(ref tool) = adb {
                match tool.discover(&stop) {
                    Ok(services) => {
                        discovery.update(services, Instant::now());
                        view.discovery_notice.clear();
                        view.searched = true;
                    }
                    Err(error) => { view.discovery_notice = error; }
                }
                discovery.show(&mut view);
                finish_pair(tool, &mut view, &mut pending_pair, &mut names, &discovery, &stop);
                describe(&mut view);
                publish(view.clone());
            }
            discovery_at = Instant::now();
        }
        if let Some(ref mut active) = tracker {
            match active.poll() {
                Ok(Some(devices)) => {
                    if let Some(ref tool) = adb {
                        set_devices(tool, &mut view, devices, &mut names, &stop);
                        discovery.show(&mut view);
                        finish_pair(tool, &mut view, &mut pending_pair, &mut names, &discovery, &stop);
                    }
                    view.server_notice.clear();
                    describe(&mut view);
                    publish(view.clone());
                }
                Ok(None) => {}
                Err(error) => {
                    tracker = None;
                    retry = Instant::now();
                    // A lost subscription does not prove that the phones disconnected.
                    // Recover this read-only channel without interrupting the user's server.
                    view.server_notice = tf("{} 연결 상태를 다시 확인하고 있어요.", &[&error]);
                    describe(&mut view);
                    publish(view.clone());
                }
            }
        } else if retry.elapsed() >= Duration::from_secs(5) {
            if let Some(ref tool) = adb {
                // 조회로만 재접속한다. 사용자 서버를 자동으로 재시작하지 않는다.
                if let Ok(tracked) = tool.track() {
                    tracker = Some(tracked);
                    view.server_notice.clear();
                    publish(view.clone());
                }
            }
            retry = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pairing_can_reuse_a_verified_address_after_discovery_expires() {
        let address: SocketAddr = "192.168.1.3:44444".parse().unwrap();
        let mut view = View { devices: crate::model::devices("192.168.1.3:44444 device model:Phone\n"), ..View::default() };
        let mut discovery = Discovery::default();
        discovery.show(&mut view);
        view.devices.clear();
        let later = Instant::now() + Duration::from_secs(121);
        discovery.update(vec![], later);
        discovery.show(&mut view);
        assert!(view.services.is_empty());
        assert_eq!(discovery.connection_address(address.ip()), Some(address));
        discovery.update(crate::model::services("phone _adb-tls-connect._tcp 192.168.1.3:55555\n"), later);
        assert_eq!(discovery.connection_address(address.ip()).unwrap().port(), 55555);
        assert_eq!(discovery.connection_address("192.168.1.4".parse().unwrap()), None);
    }
}
