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
# 배포 바이너리에 빌드 머신의 경로(사용자 홈, cargo 레지스트리, 임시 target)가 남지 않도록
# Windows 스크립트와 같은 세 접두어를 지운다. strip은 디버그 심볼만 제거하고 코드에
# 포함된 경로 문자열은 남기므로 remap이 필요하다.
cargo_home="${CARGO_HOME:-$HOME/.cargo}"
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=${cargo_home}=/cargo --remap-path-prefix=${CARGO_TARGET_DIR}=/target --remap-path-prefix=${PWD}=/adb-on"
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
  # Developer ID 서명만으로는 Gatekeeper를 통과하지 못하므로 공증까지 마쳐야 배포본이다.
  # `xcrun notarytool store-credentials <프로필>`로 저장한 keychain 프로필 이름을
  # APPLE_NOTARY_PROFILE에 주면 제출·대기·staple을 이어서 수행한다.
  if [[ -n "${APPLE_NOTARY_PROFILE:-}" ]]; then
    notary_dir="$(mktemp -d)"
    ditto -c -k --keepParent "$app" "$notary_dir/adb-on-notary.zip"
    xcrun notarytool submit "$notary_dir/adb-on-notary.zip" \
      --keychain-profile "$APPLE_NOTARY_PROFILE" --wait
    xcrun stapler staple "$app"
    xcrun stapler validate "$app"
    spctl --assess --type execute --verbose=2 "$app"
    rm -rf "$notary_dir"
  else
    echo 'Developer ID 서명은 했지만 공증하지 않았습니다. APPLE_NOTARY_PROFILE을 설정하면 공증·staple까지 수행합니다.'
  fi
else
  codesign --force --sign - "$app"
  echo '개발용 임시 서명입니다. Developer ID 서명·공증과 실제 기기 검증은 별도입니다.'
fi
archive="dist/adb-on-macos-$(uname -m).zip"
ditto -c -k --sequesterRsrc --keepParent "$app" "$archive"
# sha256 파일에는 경로 없이 파일명만 적어, 다운로드한 곳에서 `shasum -c`가 바로 통과하게 한다.
(cd "$(dirname "$archive")" && shasum -a 256 "$(basename "$archive")" > "$(basename "$archive").sha256")
echo "산출물: $archive"
