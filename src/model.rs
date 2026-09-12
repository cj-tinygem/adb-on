use crate::i18n::{t};
use std::net::SocketAddr;

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub serial: String,
    pub name: String,
    pub state: String,
}
pub fn devices(text: &str) -> Vec<Device> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let serial = parts.next()?;
            let state = parts.next()?;
            if !matches!(
                state,
                "device"
                    | "unauthorized"
                    | "offline"
                    | "recovery"
                    | "sideload"
                    | "bootloader"
                    | "no"
            ) {
                return None;
            }
            let name = parts
                .find_map(|part| part.strip_prefix("model:"))
                .unwrap_or(serial)
                .replace('_', " ");
            Some(Device {
                serial: serial.into(),
                name,
                state: state.into(),
            })
        })
        .collect()
}
/// 네트워크로 붙은 기기만 무선으로 본다. `adb connect`는 `IP:포트`, mDNS 자동 연결은
/// `adb-…._adb-tls-connect._tcp` 형태이며 ADB 판본에 따라 끝에 마침표가 붙을 수 있다.
pub fn wireless(serial: &str) -> bool {
    serial.contains(':') || serial.contains("._adb-tls-connect._tcp")
}
/// 입력을 관대하게 정리한다: 앞뒤 공백, 전각 숫자·구두점(휴대폰 화면을 보고 IME로 치는 경우),
/// 포트 앞의 공백이나 붙임표. 점이 없는 숫자열은 원래 값을 알 수 없으므로 그대로 둔다.
fn tidy(text: &str) -> String {
    text.trim()
        .chars()
        .map(|c| match c {
            '０'..='９' => char::from_u32(c as u32 - '０' as u32 + '0' as u32).unwrap_or(c),
            '．' | '。' => '.',
            '：' => ':',
            c => c,
        })
        .collect::<String>()
}
pub fn endpoint(text: &str) -> Result<SocketAddr, String> {
    let mut tidy = tidy(text);
    // "192.168.1.5 37123", "192.168.1.5-37123", "192.168.1.5:37123"은 모두 같은 뜻으로 받는다.
    if !tidy.contains(':') {
        if let Some(i) = tidy.rfind(|c: char| c == ' ' || c == '-') {
            tidy.replace_range(i..i + 1, ":");
        }
    }
    let tidy: String = tidy.chars().filter(|c| !c.is_whitespace()).collect();
    let address: SocketAddr = tidy.parse().map_err(|_| {
        t("휴대폰에 표시된 IP 주소:포트를 입력해 주세요. 예: 192.168.1.5:37123")
    })?;
    if address.port() == 0 || address.ip().is_unspecified() || address.ip().is_multicast() {
        return Err(t("휴대폰의 IP 주소와 1~65535 사이 포트를 확인해 주세요."));
    }
    Ok(address)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    /// mDNS instance name, e.g. `adb-R3CX0A1B2C3-k2Qw9z`. Also the prefix of the serial ADB
    /// assigns when it auto-connects over mDNS.
    pub name: String,
    pub address: SocketAddr,
    pub pairing: bool,
}
impl Service {
    /// The phone's serial embedded in the instance name (`adb-<serial>-<6 random>`), if the name
    /// has that shape. Used to match the phone against a device already listed by ADB.
    pub fn serial(&self) -> Option<&str> {
        let rest = self.name.strip_prefix("adb-")?;
        let (serial, tail) = rest.rsplit_once('-')?;
        (!serial.is_empty() && tail.len() == 6).then_some(serial)
    }
}
pub fn services(text: &str) -> Vec<Service> {
    text.lines()
        .filter_map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 3 {
                return None;
            }
            let pairing = match fields[1].trim_end_matches('.') {
                "_adb-tls-pairing._tcp" => true,
                "_adb-tls-connect._tcp" => false,
                _ => return None,
            };
            Some(Service {
                name: fields[0].into(),
                address: endpoint(fields[2]).ok()?,
                pairing,
            })
        })
        .collect()
}
/// Whether `device` (an `adb devices -l` row) is the phone behind this service: either ADB
/// connected it by `IP:port` or auto-connected it by its mDNS name, or its USB serial matches.
pub fn same_phone(service: &Service, device: &Device) -> bool {
    device.serial == service.address.to_string()
        || device.serial.starts_with(&format!("{}.", service.name))
        || service.serial().is_some_and(|s| s == device.serial)
}
pub fn pairing_code(code: &str) -> Result<String, String> {
    // 공백·붙임표로 끊어 적거나 전각 숫자로 쳐도 받는다.
    let code: String = tidy(code).chars().filter(|c| !c.is_whitespace() && *c != '-').collect();
    if code.len() == 6 && code.bytes().all(|b| b.is_ascii_digit()) {
        Ok(code)
    } else {
        Err(t("휴대폰에 표시된 숫자 6자리를 입력해 주세요."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_authorized_and_pending_without_treating_header_as_device() {
        let rows = devices(
            "List of devices attached\r\nR5ABC unauthorized usb:1-1\r\n192.168.1.2:39411 device product:x model:Pixel_9 device:tokay transport_id:2\r\n",
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].state, "unauthorized");
        assert_eq!(rows[1].name, "Pixel 9");
    }
    #[test]
    fn wireless_covers_tcp_and_mdns_serials_but_not_usb() {
        for serial in [
            "192.168.1.2:39411",
            "[fe80::1]:5555",
            "adb-R5ABC-yF7kj8._adb-tls-connect._tcp",
            "adb-R5ABC-yF7kj8._adb-tls-connect._tcp.",
        ] {
            assert!(wireless(serial), "{serial}");
        }
        for serial in ["R5ABC", "emulator-5554", "adb-R5ABC._adb-tls-pairing._tcp"] {
            assert!(!wireless(serial), "{serial}");
        }
    }
    #[test]
    fn endpoints_are_addresses_not_commands_and_reject_zero_port() {
        assert!(endpoint("192.168.1.2:37001").is_ok());
        assert_eq!(endpoint(" 192.168.1.2 37001 ").unwrap().port(), 37001);
        assert_eq!(endpoint("192.168.1.2-37001").unwrap().port(), 37001);
        assert_eq!(endpoint("１９２．１６８．１．２：３７００１").unwrap().port(), 37001);
        assert!(endpoint("19216812:37001").is_err(), "digits without dots cannot be reconstructed");
        assert!(endpoint("[fe80::1]:37001").is_ok());
        for text in ["-a", "host:1234;cmd", "192.168.1.2", "127.0.0.1:0"] {
            assert!(endpoint(text).is_err(), "{text}");
        }
    }
    #[test]
    fn pairing_and_connection_ports_stay_separate() {
        let rows = services(
            "List of discovered mdns services\nphone _adb-tls-pairing._tcp. 192.168.1.4:33333\nphone _adb-tls-connect._tcp 192.168.1.4:44444\nother _http._tcp 192.168.1.4:80\n",
        );
        assert_eq!(rows.len(), 2);
        assert!(rows[0].pairing);
        assert!(!rows[1].pairing);
        assert_ne!(rows[0].address.port(), rows[1].address.port());
        assert_eq!(rows[0].name, "phone");
        let found = services("adb-R3CX0A1B2C3-k2Qw9z _adb-tls-connect._tcp 192.168.1.20:39769\n")
            .remove(0);
        assert_eq!(found.serial(), Some("R3CX0A1B2C3"));
        assert_eq!(rows[0].serial(), None);
        let device = |serial: &str| Device { serial: serial.into(), name: "SM S908N".into(), state: "device".into() };
        assert!(same_phone(&found, &device("192.168.1.20:39769")));
        assert!(same_phone(&found, &device("adb-R3CX0A1B2C3-k2Qw9z._adb-tls-connect._tcp")));
        assert!(same_phone(&found, &device("R3CX0A1B2C3")));
        assert!(!same_phone(&found, &device("192.168.1.20:41839")));
        assert!(!same_phone(&found, &device("R3CX0A1B2C4")));
        assert!(pairing_code("123456").is_ok());
        assert_eq!(pairing_code(" 12 34 56 ").unwrap(), "123456");
        assert_eq!(pairing_code("１２３４５６").unwrap(), "123456");
        assert_eq!(pairing_code("123-456").unwrap(), "123456");
        // 전각 숫자와 사이 공백은 이제 받는다. 길이가 맞지 않거나 숫자가 아닌 것만 거른다.
        for bad in ["", "12345", "1234567", "12 456", "123\n45", "12345a"] {
            assert!(pairing_code(bad).is_err());
        }
    }
}
