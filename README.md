<p align="center"><em><a href="README.md">English</a> ∙ <a href="README-zh-Hans.md">简体中文</a> ∙ <a href="README-ja.md">日本語</a> ∙ <a href="README-es.md">Español</a> ∙ <a href="README-ko.md">한국어</a> ∙ <a href="README-de.md">Deutsch</a> ∙ <a href="README-fr.md">Français</a></em></p>

<br>

<div align="center">
  <img src="docs/assets/icon.svg" width="88" alt="adb-on icon representing a phone connection">
  <h1>adb-on</h1>
  <p><strong>Phone connection, in one window.</strong></p>
  <p>A small, simple desktop tool for connecting an Android phone without opening a terminal.</p>
  <p>Runs on Windows. A macOS build is being prepared.</p>
</div>

> **Windows version available.** Download `adb-on-windows-x64.zip` from Releases, unzip it, and open `adb-on.exe`. Connection to a real phone has been verified over USB and Wi-Fi. macOS builds are not published yet.

<p align="center"><img src="docs/assets/windows.png" width="400" alt="Actual connection waiting screen of adb-on running on Windows"></p>

**What you want to do is simple.** Open the app and connect your phone. adb-on handles what needs to be done on the PC, and tells you when something has to be tapped on the phone.

<br>

## Table of Contents

- [Why adb-on?](#why-adb-on)
- [Downloads and Support Status](#downloads-and-support-status)
- [First Connection over USB](#first-connection-over-usb)
- [Connecting Without a Cable](#connecting-without-a-cable)
- [Reconnecting Next Time](#reconnecting-next-time)
- [Quick Fixes When Stuck](#quick-fixes-when-stuck)
- [What Does It Do Automatically?](#what-does-it-do-automatically)
- [What Small and Fast Means](#what-small-and-fast-means)
- [What Stays on My PC?](#what-stays-on-my-pc)
- [Frequently Asked Questions](#frequently-asked-questions)
- [Disclaimer](#disclaimer)
- [Reporting Issues and Terms of Use](#reporting-issues-and-terms-of-use)
- [Official References](#official-references)

<br>

## Why adb-on?

You built an Android app, and then got stuck the moment you tried to connect it to your phone?

Locating the `adb` installation, copying commands, and re-entering a changed port are not the core of building an app. adb-on puts only what is needed for the connection into a small window.

- **USB is checked automatically:** Distinguishes connected, waiting for phone approval, and not responding.
- **Wireless follows the guide:** Select a discovered address and enter the 6-digit code from the phone. If automatic discovery does not work, you can enter the address manually.
- **Step by step the first time:** Guides you from Developer options to the Allow button on the phone.
- **Works with existing tools:** Uses the default ADB server. It does not unconditionally kill an existing server or delete authentication keys.
- **Only what is needed:** Focuses on the connection, without cloud servers, mirroring, or a file manager.

<br>

## Downloads and Support Status

Official distribution location: [adb-on downloads and releases](https://github.com/cj-tinygem/adb-on/releases)

|Platform|Provided As|Current Status|
|---|---|---|
|Windows x64|`adb-on.exe`|Available. Verified with a real phone over USB and Wi-Fi. Pairing with a code is still awaiting real-phone verification.|
|macOS Apple Silicon|`adb-on.app`|Not published yet. Build path prepared, not verified on an actual Mac.|
|macOS Intel|`adb-on.app`|Not published yet. Build path prepared, not verified on an actual Mac.|

- There is no installer. Unzip, put the single `adb-on.exe` in any folder, and run it. The license files in the zip are not needed to run the app.
- The Windows app does not require WebView2, Node.js, or Java.
- The app language follows the PC language at first. You can switch among English, 简体中文, 日本語, Español, 한국어, Deutsch, and Français in Tool settings.
- If ADB is already present, it is reused. If not, the app prepares Google's official tools after you accept the terms. An internet connection is required for the initial preparation.
- The current development build is not a distribution that has completed public code signing or notarization. It does not automatically dismiss OS warnings or change security settings.
- The target minimum macOS version is 12. The actual compatibility range will be confirmed after native verification.
- Native support for Linux and Windows ARM is not currently provided.

Each release includes the **SHA-256 checksum** of the executable and the scope of verification. Check the release notes so you do not confuse development test builds with officially supported versions.

<br>

## First Connection over USB

### 1. Open adb-on

If a message says ADB is not found, press **Prepare first connection**. Review and accept the Google SDK terms, and the app downloads and prepares the official files.

If Android Studio is already installed, this step is usually skipped. If you installed the tools in a different location, you can select `adb.exe` or `adb` from **Tool settings → Select an existing ADB file**.

### 2. Open Developer options on the phone

|Phone|Menu to Find|
|---|---|
|Galaxy|Settings → About phone → Software information → tap **Build number** 7 times|
|Pixel and others|Settings → About phone → tap **Build number** 7 times|

If a lock screen password is requested, enter it **directly on the phone**. Menu names may differ depending on the model and Android version.

### 3. Turn on USB debugging

Turn on **Developer options → USB debugging** in the phone settings. Due to Android security, this must be done directly on the phone.

### 4. Connect with a data cable and allow

Unlock the phone and tap **Allow USB debugging? → Allow**. If it is your own PC, selecting 'Always allow' makes the next connection easier.

When adb-on changes to **Connected**, select the phone in the development tools on this PC.

> Charge-only cables cannot be used for an ADB connection. A phone that has not granted permission is not forcibly approved from the PC.

<br>

## Connecting Without a Cable

Uses wireless debugging, which is supported on Android 11 and later.

### 1. Connect to the same Wi-Fi

The PC and the phone must be on a network where they can communicate with each other. Corporate, school, and guest Wi-Fi may block communication between devices.

### 2. Open the pairing screen on the phone

Select **Settings → Developer options → Wireless debugging → Pair device with pairing code**. Keep this screen open.

### 3. Press Wireless connection in adb-on

Tap the discovered pairing address, enter the **6-digit number** shown on the phone, and press **Pair**. If only one address is discovered, it is filled in automatically. If several phones are visible, compare with the address shown on your phone's screen.

Automatic discovery continues for a while. If nothing is found, you can enter the **IP address:port** shown on the phone manually.

### 4. Check the connection status

Pairing is the process of registering the PC as trusted, and is different from completing the connection.

If the connection does not work, go back one screen on the phone, enter the **IP address and port on the first Wireless debugging screen** into the connection field in adb-on, and press **Connect**.

|Address|Where Is It?|Where Does It Go?|
|---|---|---|
|Pairing address|'Pair device with pairing code' popup|① Pairing address field|
|Connection address|First Wireless debugging screen|② Connection address field|

**The two ports are different.** For example, pairing may be `192.168.1.5:37123` and connection may be `192.168.1.5:39511`. Do not copy the example; use the values from your phone.

<br>

## Reconnecting Next Time

- **USB:** If the PC was trusted before, plug in the cable and check the status. If approval is required again, allow it on the phone.
- **Wireless:** If wireless debugging is on in the same network and ADB discovers the device, it reconnects. If automatic connection does not work, use the phone's **current connection address**.
- **After a reboot or Wi-Fi change:** Wireless debugging may be turned off, or the IP and port may change. adb-on does not store a stale address and keep retrying it.
- **Closing the app:** adb-on exits when the window is closed. The ADB server and connections used by other development tools are kept. There is no hidden adb-on resident service.
- **Disconnecting wireless:** Press the disconnect button for that device. Since ADB may reconnect automatically, turn off wireless debugging on the phone to make sure the connection stays closed.

<br>

## Quick Fixes When Stuck

|What You See|What to Do First|
|---|---|
|Waiting for approval on the phone|Unlock the phone and check the Allow USB debugging popup|
|USB is plugged in but the phone is not visible|Check in this order: data cable, another port, USB debugging|
|Still not visible on Windows|Check the [OEM USB driver guide](https://developer.android.com/studio/run/oem-usb)|
|Wireless device is not discovered|Keep the pairing popup open or enter the address manually|
|Pairing code rejected|Open a new pairing popup on the phone and enter the new code and address|
|Paired but not connected|Connect using the **connection address** on the first Wireless debugging screen|
|Does not work on corporate Wi-Fi|Communication may be blocked even on the same network. Use a USB connection|
|Notice about a different ADB server version|Select the same ADB file as the development tool you are using|
|Development tool cannot find the phone|Check that it uses the default ADB server in the same PC environment. WSL is a separate environment|
|ADB download failed|Connect to the internet and prepare again, or select an existing ADB file|

The same guidance is available step by step in **USB connection guide** inside the app. An empty device list alone is not used to conclude whether the driver, cable, or phone settings are the cause.

<br>

## What Does It Do Automatically?

|Handled on the PC|Must Be Done on the Phone|
|---|---|
|Find existing ADB and check connection status|Turn on Developer options and USB/wireless debugging|
|Download official ADB and verify files after consent|Approve trusting the PC for the first time|
|Discover wireless addresses, run pairing commands, request connections|Open the pairing screen and check the 6-digit code|
|Guide the next action based on status|Resolve phone permission issues such as corporate management policies|

Screen mirroring, file management, app building, rooting, and automatic driver installation are not provided.

<br>

## What Small and Fast Means

Performance is not claimed based on a small executable alone. The following items are measured before distribution.

- Time from launch until the window is displayed.
- Idle CPU and memory. The existing shared ADB server is measured separately.
- Size of the executable and the initial download.
- Status updates when the phone is connected or disconnected, and screen responsiveness during interaction.

The GUI is built with **Rust + Slint** and does not launch a system WebView. State changes use ADB's notifications, and wireless discovery queries run only for a limited time when the user opens the wireless connection.

Windows development build measurements (no phone connected, 2026-09-11):

|Item|Result|
|---|---|
|Executable|About 12.0MiB|
|Idle memory|About 126MiB working set, about 59MiB private (GPU drawing; most of the working set is graphics driver memory)|
|Window display across three relaunches|0.47~0.57 seconds|
|Shared ADB server|About 4.7MiB separately, existing process kept|

In each run, 30 seconds of idle used 0.13~0.36 seconds of CPU time (about 0.4~1.2% of one core). The window is drawn with the GPU since this build; the software-drawing fallback in the FAQ uses about 30MiB but scrolls less smoothly. The first launch of the earlier initial build took 1.43 seconds, and cold boot or performance on other PCs is not guaranteed. Load during device connection and macOS performance have not yet been verified.

<br>

## What Stays on My PC?

- The ADB location setting (`settings.json`) and, if you chose in-app preparation, a copy of the official ADB. Both live only in the folder below.
  - Windows: `%LOCALAPPDATA%\tinygem\adb-on\data\`
  - macOS: `~/Library/Application Support/ai.tinygem.adb-on/`
- The PC authentication keys and server managed by ADB itself. adb-on does not delete or replace existing keys.

Pairing codes are not stored in settings or usage logs. There is no remote storage server. During automatic preparation, the app connects to Google's download server, and guide links open in the default browser.

To stop using the app, choose **Tool settings → Delete adb-on data**, then close the window and delete the executable. If you already deleted the executable, delete the folder above yourself. If the ADB prepared here is running as the server, it is stopped during deletion; other tools' ADB and the ADB authentication keys are left alone. Deleting the executable by itself does not remove this folder.

<br>

## Frequently Asked Questions

### Do I need to install it?

No. Put the single `adb-on.exe` in any folder and run it. Settings and the prepared ADB are stored in the user data folder above and can be deleted from Tool settings.

### Is it open source?

Yes. The source code is published under the MIT license. Third-party components follow their respective licenses.

### Do I need to know ADB?

No commands are needed for a basic connection. The approvals and settings that must be done on the phone are shown on screen.

### Can I develop apps without Android Studio?

adb-on is a connection tool. It does not replace the SDK and build tools needed to build an app.

### If I connect on Windows, does it work in WSL too?

It is not passed through automatically. Windows and WSL may be separate development environments.

### Does macOS support both Intel and Apple Silicon?

Build paths for both architectures are prepared, but no macOS build is published yet and none has been verified on an actual Mac. When one is published it will not carry an Apple Developer ID signature, so the first launch will need right-click → Open, once.

### Do I need to install a separate app on the phone?

No separate app is installed on the phone for the adb-on connection. Android's debugging feature is used.

<br>

## Disclaimer

adb-on is open-source software released under the MIT License. The license text in [LICENSE.txt](LICENSE.txt) is the binding terms; the points below restate it in plain language.

- **No warranty.** The software is provided "as is", without warranty of any kind, express or implied. There is no guarantee that it fits a particular purpose or works without errors.
- **No liability.** The developers and contributors are not liable for any damage arising from the use of, or inability to use, this software. This includes data loss, device malfunction, failed connections, business interruption, and any other direct or indirect damage. Whether and how you use it is entirely your own decision and responsibility.
- **You manage the risks of developer options.** Developer options and USB/wireless debugging are Android features. While they are on, a connected PC can control the phone. Do not enable them on PCs or networks you do not trust, and turn them off when you are done. adb-on does not manage these settings for you.
- **Third-party tools follow their own terms.** ADB is Google's Android SDK Platform-Tools and is provided under Google's terms. adb-on does not accept those terms on your behalf. Other components follow the licenses listed in [Third-Party Notices](THIRD-PARTY-NOTICES.md).
- **Not affiliated with Google.** adb-on is not affiliated with, sponsored by, or endorsed by Google or Android. Android is a trademark of Google LLC.
- **No support commitment.** Features may change or distribution may stop without notice. Updates, troubleshooting, and continued compatibility are not promised.
- **Applies as far as the law allows.** These limitations do not apply to liability that applicable law does not allow to be excluded.

### The window looks wrong or scrolling stutters over Remote Desktop

adb-on draws with the GPU by default and switches to software drawing on its own when the graphics driver cannot provide OpenGL. If the window still looks wrong, or feels slow in a Remote Desktop or virtual machine session, start it with software drawing forced:

```
cmd /c "set SLINT_BACKEND=winit-software && adb-on.exe"
```

Run this in the folder that contains adb-on.exe. Nothing is written to disk. On macOS, set the same environment variable in Terminal before launching the app.

<br>

## Reporting Issues and Terms of Use

Please include the following in an [issue report](https://github.com/cj-tinygem/adb-on/issues).

- PC operating system, phone model, and Android version.
- Whether you used USB or wireless.
- At which step the issue occurred and what message was shown.

**Do not send pairing codes, PC authentication keys, or personal information.** Also mask personal information in screenshots.

[License](LICENSE.txt) · [Third-Party Notices](THIRD-PARTY-NOTICES.md) · Made by cj at tinygem

<br>

## Official References

- [Android ADB official documentation](https://developer.android.com/tools/adb)
- [Run apps on a hardware device](https://developer.android.com/studio/run/device)
- [Official Platform-Tools](https://developer.android.com/tools/releases/platform-tools)
- [Windows OEM USB drivers](https://developer.android.com/studio/run/oem-usb)
- [Slint](https://slint.dev)

<p align="center"><a href="https://slint.dev"><img src="https://github.com/slint-ui/slint/raw/master/logo/MadeWithSlint-logo-light.svg" width="160" alt="Made with Slint"></a></p>

The document structure continues to reference the table of contents, visual explanations, and navigability of [System Design Primer](https://github.com/donnemartin/system-design-primer). To suit the purpose of adb-on, it provides both a short getting-started path and detailed troubleshooting.
