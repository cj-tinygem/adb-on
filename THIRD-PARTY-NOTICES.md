# Third-Party Notices

adb-on is built on the projects below. The app's own MIT license does not limit the separate rights of these components.

|Component|Purpose|License / source|
|---|---|---|
|Slint 1.17.1|GUI and native rendering|[Royalty-free license](https://slint.dev/agreements/slint-royalty-free-license.pdf), [Slint](https://slint.dev)|
|Rust standard library|Native core|MIT or Apache-2.0, https://www.rust-lang.org/policies/licenses|
|serde / serde_json|Settings and diagnostics format|MIT or Apache-2.0|
|directories|Per-OS user folders|MIT or Apache-2.0|
|open|Opening the fixed official guide links|MIT|
|rfd|Picking an existing ADB file|MIT|
|ureq / rustls|HTTPS download of the official tools|Each crate's MIT/Apache-2.0/ISC choice|
|zip|Extracting the required files from the verified SDK|MIT|
|sha2|SHA-256 check of the official download|MIT or Apache-2.0|
|libc (Unix only)|Non-blocking setup of child output pipes|MIT or Apache-2.0|
|windows-sys (Windows only)|Querying available pipe bytes, hiding the console window|MIT or Apache-2.0|

Slint is used under the Royalty-free License 2.0 for desktop apps. The app shows "Made with Slint" with the official link at the bottom. Distributions include the full license texts of the Rust dependencies. The exact locked versions are defined by Cargo.lock. The list of runtime dependencies and their license texts is in licenses/INDEX.md inside the distribution. option-ext (MPL-2.0) is used unmodified, and its source archive (.crate) is bundled as well.

ADB is not part of the adb-on executable. The app either uses an existing SDK on the user's machine or, once the user accepts the Google SDK terms, downloads the official Platform-Tools 37.0.1 from https://dl.google.com. NOTICE.txt from the Google distribution is preserved during extraction. Download details and verification hashes are pinned in scripts/platform-tools.json.

Reference for the README structure: [System Design Primer](https://github.com/donnemartin/system-design-primer). No text or diagrams are copied; the content is written for this project.

Windows release builds link the Microsoft C runtime statically. See the Microsoft redistribution terms at https://learn.microsoft.com/cpp/windows/redistributing-visual-cpp-files. Because a runtime update installed on the system does not change the copy embedded in the app, related security fixes must be picked up by a new app build.
