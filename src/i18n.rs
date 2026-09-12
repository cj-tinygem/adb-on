//! 사용자에게 보이는 문장의 언어 선택. 한국어 원문을 키로 `translations/messages.json`에서 찾는다.
//! UI(`.slint`)의 문장은 같은 JSON에서 build.rs가 만든 번들 번역을 쓰고, Rust 쪽 문장은 여기서 고른다.
use std::{
    collections::HashMap,
    sync::{
        OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

/// 지원 언어. README 언어 목록과 같은 순서이며, 코드는 번들 번역 폴더 이름과 같다.
pub const LANGUAGES: [(&str, &str); 7] = [
    ("en", "English"),
    ("zh-Hans", "简体中文"),
    ("ja", "日本語"),
    ("es", "Español"),
    ("ko", "한국어"),
    ("de", "Deutsch"),
    ("fr", "Français"),
];
const SOURCE_INDEX: usize = 4; // 한국어 원문

static CURRENT: AtomicUsize = AtomicUsize::new(SOURCE_INDEX);
static TABLE: OnceLock<HashMap<&'static str, HashMap<&'static str, &'static str>>> = OnceLock::new();

fn table() -> &'static HashMap<&'static str, HashMap<&'static str, &'static str>> {
    TABLE.get_or_init(|| {
        let raw: &'static str = include_str!("../translations/messages.json");
        let parsed: serde_json::Value = serde_json::from_str(raw).expect("translations/messages.json");
        let mut map = HashMap::new();
        if let Some(strings) = parsed["strings"].as_object() {
            for (key, langs) in strings {
                let key: &'static str = Box::leak(key.clone().into_boxed_str());
                let mut inner = HashMap::new();
                if let Some(obj) = langs.as_object() {
                    for (code, text) in obj {
                        if let Some(text) = text.as_str().filter(|t| !t.is_empty()) {
                            inner.insert(
                                Box::leak(code.clone().into_boxed_str()) as &'static str,
                                Box::leak(text.to_string().into_boxed_str()) as &'static str,
                            );
                        }
                    }
                }
                map.insert(key, inner);
            }
        }
        map
    })
}

/// 현재 언어 코드.
pub fn current() -> &'static str {
    LANGUAGES[CURRENT.load(Ordering::Relaxed)].0
}
/// 현재 언어의 목록 인덱스(UI 콤보박스와 같은 순서).
pub fn index() -> usize {
    CURRENT.load(Ordering::Relaxed)
}
/// 언어 코드를 현재 언어로 설정한다. 모르는 코드는 무시하고 false를 돌려준다.
pub fn set(code: &str) -> bool {
    match LANGUAGES.iter().position(|(c, _)| *c == code) {
        Some(i) => {
            CURRENT.store(i, Ordering::Relaxed);
            true
        }
        None => false,
    }
}
/// 언어별 UI 글꼴. 시스템 대체 글꼴은 로캘을 모른 채 한자를 고르므로(한국어 PC에서 중국어 간체가 빈 칸으로
/// 보였다), 각 언어의 글자를 모두 가진 OS 기본 글꼴을 직접 지정한다. 빈 문자열은 툴킷 기본값이다.
pub fn font_family(code: &str) -> &'static str {
    if cfg!(target_os = "windows") {
        match code {
            "zh-Hans" => "Microsoft YaHei UI",
            "ja" => "Yu Gothic UI",
            "ko" => "Malgun Gothic",
            _ => "Segoe UI",
        }
    } else if cfg!(target_os = "macos") {
        match code {
            "zh-Hans" => "PingFang SC",
            "ja" => "Hiragino Sans",
            "ko" => "Apple SD Gothic Neo",
            _ => "",
        }
    } else {
        ""
    }
}

/// OS 로캘 태그(예: `ko-KR`, `zh-CN`, `en_US`)를 지원 언어 코드로 고른다. 없으면 영어.
pub fn detect(tag: Option<&str>) -> &'static str {
    let tag = tag.unwrap_or("").replace('_', "-").to_ascii_lowercase();
    let primary = tag.split('-').next().unwrap_or("");
    match primary {
        "ko" => "ko",
        "zh" => "zh-Hans",
        "ja" => "ja",
        "es" => "es",
        "de" => "de",
        "fr" => "fr",
        _ => "en",
    }
}
/// 한국어 원문을 현재 언어로 옮긴다. 번역이 없으면 원문을 그대로 쓴다.
pub fn t(source: &str) -> String {
    let code = current();
    if code == LANGUAGES[SOURCE_INDEX].0 {
        return source.to_string();
    }
    table()
        .get(source)
        .and_then(|m| m.get(code))
        .map(|s| s.to_string())
        .unwrap_or_else(|| source.to_string())
}
/// `{}` 자리에 값을 순서대로 넣는다. 번역문이 자리 표시자를 잃어도 값은 뒤에 덧붙여 남긴다.
pub fn tf(source: &str, args: &[&str]) -> String {
    let mut out = t(source);
    for arg in args {
        if let Some(pos) = out.find("{}") {
            out.replace_range(pos..pos + 2, arg);
        } else {
            out.push(' ');
            out.push_str(arg);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_maps_locale_tags_and_falls_back_to_english() {
        assert_eq!(detect(Some("ko-KR")), "ko");
        assert_eq!(detect(Some("zh_CN")), "zh-Hans");
        assert_eq!(detect(Some("zh-TW")), "zh-Hans");
        assert_eq!(detect(Some("pt-BR")), "en");
        assert_eq!(detect(None), "en");
    }

    #[test]
    fn tf_fills_placeholders_in_order_and_keeps_extra_args() {
        assert_eq!(tf("{} 파일을 선택해 주세요.", &["adb.exe"]), "adb.exe 파일을 선택해 주세요.");
        assert_eq!(tf("지울 데이터가 없어요.\n{}", &["C:\\x", "extra"]), "지울 데이터가 없어요.\nC:\\x extra");
    }

    #[test]
    fn every_source_key_has_all_languages_once_translated() {
        let map = table();
        assert!(map.len() > 100, "messages.json must be loaded");
        for (key, langs) in map {
            for (code, _) in LANGUAGES.iter().filter(|(c, _)| *c != "ko") {
                assert!(langs.contains_key(code), "missing {code} for {key:?}");
            }
        }
    }
}
