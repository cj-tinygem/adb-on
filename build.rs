fn main() {
    println!("cargo:rerun-if-changed=translations/messages.json");
    println!("cargo:rerun-if-changed=docs/assets/icon.ico");
    #[cfg(target_os = "windows")]
    {
        // Explorer and the taskbar show the icon embedded in the executable, not the window icon.
        let mut res = winresource::WindowsResource::new();
        res.set_icon("docs/assets/icon.ico");
        res.compile().expect("Windows resource (icon)");
    }
    println!("cargo:rerun-if-changed=ui/app.slint");
    #[cfg(feature = "gui")]
    {
        // translations/messages.json 하나가 UI와 Rust 문장의 정본이다. UI용 .po는 빌드 때 만든다.
        let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"))
            .join("translations");
        let raw = std::fs::read_to_string("translations/messages.json").expect("messages.json");
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("messages.json JSON");
        let languages = parsed["languages"].as_array().expect("languages");
        let strings = parsed["strings"].as_object().expect("strings");
        let domain = std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME");
        fn po_escape(s: &str) -> String {
            s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
        }
        for lang in languages {
            let lang = lang.as_str().expect("language code");
            let dir = out.join(lang).join("LC_MESSAGES");
            std::fs::create_dir_all(&dir).expect("translation dir");
            let mut po = String::from("msgid \"\"\nmsgstr \"\"\n\"Content-Type: text/plain; charset=UTF-8\\n\"\n\n");
            for (key, langs) in strings {
                let text = langs[lang].as_str().unwrap_or("");
                if text.is_empty() {
                    continue;
                }
                po.push_str(&format!("msgid \"{}\"\nmsgstr \"{}\"\n\n", po_escape(key), po_escape(text)));
            }
            std::fs::write(dir.join(format!("{domain}.po")), po).expect("write .po");
        }
        let config = slint_build::CompilerConfiguration::new()
            .with_bundled_translations(&out)
            .with_default_translation_context(slint_build::DefaultTranslationContext::None);
        slint_build::compile_with_config("ui/app.slint", config).expect("GUI 컴파일 실패");
    }
}
