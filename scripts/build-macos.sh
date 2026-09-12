#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "$(uname -s)" != Darwin ]]; then
  echo 'macOS에서 실행해 주세요. 이 결과는 macOS 빌드·실행 통과가 아닙니다.' >&2
  exit 2
fi
if [[ "${1:-}" == --self-test ]]; then
  command -v cargo >/dev/null
  command -v xcrun >/dev/null
  test -f Cargo.toml
  test -f docs/assets/icon.icns
  printf '{"status":"ok","platform":"macos","writes_release":false,"starts_gui":false}\n'
  exit 0
fi
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/adb-on-target}"
export MACOSX_DEPLOYMENT_TARGET=12.0
export CARGO_BUILD_JOBS=1
export CARGO_INCREMENTAL=0
export SLINT_STYLE=fluent-light
cargo build --release --locked
# 앱 버전은 Cargo.toml 하나만 소유한다. `cargo pkgid`는 `…#0.1.0` 또는 `…@0.1.0` 형식이다.
version="$(cargo pkgid --locked | sed -E 's/.*[#@]//')"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]] || { echo "버전을 읽지 못했습니다: $version" >&2; exit 1; }
app='dist/macos/adb-on.app'
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$CARGO_TARGET_DIR/release/adb-on" "$app/Contents/MacOS/adb-on"
cp LICENSE.txt THIRD-PARTY-NOTICES.md "$app/Contents/Resources/"
cp docs/assets/icon.icns "$app/Contents/Resources/adb-on.icns"
python3 scripts/export-licenses.py "$app/Contents/Resources/licenses"
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>adb-on</string>
<key>CFBundleIdentifier</key><string>ai.tinygem.adb-on</string>
<key>CFBundleName</key><string>adb-on</string>
<key>CFBundleDisplayName</key><string>adb-on</string>
<key>CFBundleIconFile</key><string>adb-on</string>
<key>LSApplicationCategoryType</key><string>public.app-category.developer-tools</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>${version}</string>
<key>CFBundleVersion</key><string>1</string>
<key>LSMinimumSystemVersion</key><string>12.0</string>
<key>NSHighResolutionCapable</key><true/>
<key>NSHumanReadableCopyright</key><string>cj (tinygem) · MIT License</string>
</dict></plist>
PLIST
plutil -lint "$app/Contents/Info.plist"
"$app/Contents/MacOS/adb-on" --self-test
if [[ -n "${APPLE_SIGNING_IDENTITY:-}" ]]; then
  codesign --force --options runtime --timestamp --sign "$APPLE_SIGNING_IDENTITY" "$app"
  codesign --verify --deep --strict "$app"
else
  codesign --force --sign - "$app"
  echo '개발용 임시 서명입니다. Developer ID 서명·공증과 실제 기기 검증은 별도입니다.'
fi
archive="dist/adb-on-macos-$(uname -m).zip"
ditto -c -k --sequesterRsrc --keepParent "$app" "$archive"
shasum -a 256 "$archive" > "$archive.sha256"
echo "산출물: $archive"
