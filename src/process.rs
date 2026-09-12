use crate::i18n::{t};
use std::{
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
const LIMIT: usize = 256 * 1024;

#[cfg(unix)]
fn prepare(pipe: &impl std::os::fd::AsRawFd) -> std::io::Result<()> {
    let fd = pipe.as_raw_fd();
    // 소유한 파이프만 비차단으로 바꾼다. 공유 ADB 서버의 핸들은 변경하지 않는다.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}
#[cfg(windows)]
fn prepare(_: &impl std::os::windows::io::AsRawHandle) -> std::io::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn drain(pipe: &mut impl Read, bytes: &mut Vec<u8>) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    loop {
        match pipe.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                bytes.extend_from_slice(&buffer[..n]);
                if bytes.len() > LIMIT {
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
#[cfg(windows)]
fn drain(
    pipe: &mut (impl Read + std::os::windows::io::AsRawHandle),
    bytes: &mut Vec<u8>,
) -> std::io::Result<()> {
    use windows_sys::Win32::{
        Foundation::{ERROR_BROKEN_PIPE, GetLastError},
        System::Pipes::PeekNamedPipe,
    };
    let mut buffer = [0; 4096];
    loop {
        let mut available = 0;
        // PeekNamedPipe는 익명 파이프도 지원한다. 읽을 바이트가 있을 때만 Read를 호출한다.
        let ok = unsafe {
            PeekNamedPipe(
                pipe.as_raw_handle(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                &mut available,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            if unsafe { GetLastError() } == ERROR_BROKEN_PIPE {
                break;
            }
            return Err(std::io::Error::last_os_error());
        }
        if available == 0 {
            break;
        }
        let take = (available as usize).min(buffer.len());
        let n = pipe.read(&mut buffer[..take])?;
        if n == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..n]);
        if bytes.len() > LIMIT {
            break;
        }
    }
    Ok(())
}

/// shell 없이 유한 명령을 호출한다. 자식 종료 후 다른 프로세스가 보유한 파이프 EOF를 기다리지 않는다.
pub fn run(
    path: &Path,
    args: &[&str],
    input: Option<&str>,
    stop: &Arc<AtomicBool>,
    seconds: u64,
) -> Result<String, String> {
    if stop.load(Ordering::Relaxed) {
        return Err(t("작업을 취소했습니다."));
    }
    let mut command = Command::new(path);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for key in [
        "ADB_SERVER_SOCKET",
        "ANDROID_ADB_SERVER_PORT",
        "ANDROID_ADB_SERVER_ADDRESS",
    ] {
        command.env_remove(key);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    // 제품 입력은 숫자6자리 한 줄뿐이다. 파이프 용량을 넘거나 줄을 나누는 입력은 실행 전에 거부한다.
    if input.is_some_and(|s| s.len() > 64 || s.chars().any(char::is_control)) {
        return Err(t("입력 형식 오류"));
    }
    let mut child = command.spawn().map_err(|_| {
        t("ADB를 실행하지 못했습니다. 파일 위치와 실행 권한을 확인해 주세요.")
    })?;
    let mut stdin = child.stdin.take().unwrap();
    if let Some(input) = input
        && writeln!(stdin, "{input}").is_err()
    {
        let _ = child.kill();
        let _ = child.wait();
        return Err(t("ADB 입력 전달 실패"));
    }
    drop(stdin);
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    if prepare(&stdout).and_then(|_| prepare(&stderr)).is_err() {
        let _ = child.kill();
        let _ = child.wait();
        return Err(t("ADB 출력 파이프 준비 실패"));
    }
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let start = Instant::now();
    let result = loop {
        if drain(&mut stdout, &mut output)
            .and_then(|_| drain(&mut stderr, &mut errors))
            .is_err()
        {
            break Err(t("ADB 응답 읽기 실패"));
        }
        if output.len() > LIMIT || errors.len() > LIMIT {
            break Err(t("ADB 응답이 예상보다 커서 중단했습니다."));
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                let _ = drain(&mut stdout, &mut output);
                let _ = drain(&mut stderr, &mut errors);
                break if status.success() {
                    Ok(())
                } else {
                    Err(t(
                        "ADB 요청이 완료되지 않았습니다. 휴대폰의 화면·코드·주소를 다시 확인해 주세요.",
                    ))
                };
            }
            Err(_) => break Err(t("ADB 상태를 확인하지 못했습니다.")),
            Ok(None) => {}
        }
        if stop.load(Ordering::Relaxed) {
            break Err(t("작업을 취소했습니다."));
        }
        if start.elapsed() >= Duration::from_secs(seconds) {
            break Err(t("응답을 기다리는 시간이 지났습니다. 휴대폰과 연결 상태를 확인해 주세요."));
        }
        thread::sleep(Duration::from_millis(25));
    };
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result?;
    if output.len() > LIMIT || errors.len() > LIMIT {
        return Err(t("ADB 응답 크기 초과"));
    }
    Ok(String::from_utf8_lossy(&output).into_owned())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    #[test]
    fn hung_command_is_reaped_at_deadline() {
        let start = Instant::now();
        let result = run(
            Path::new("/bin/sleep"),
            &["5"],
            None,
            &Arc::new(AtomicBool::new(false)),
            0,
        );
        assert!(result.unwrap_err().contains("시간"));
        assert!(start.elapsed() < Duration::from_secs(2));
    }
    #[test]
    fn open_pipe_without_output_does_not_wait_for_eof() {
        use std::os::fd::FromRawFd;
        let mut fds = [0; 2];
        assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
        let mut reader = unsafe { std::fs::File::from_raw_fd(fds[0]) };
        let _writer = unsafe { std::fs::File::from_raw_fd(fds[1]) };
        prepare(&reader).unwrap();
        let mut output = Vec::new();
        drain(&mut reader, &mut output).unwrap();
        assert!(output.is_empty());
    }
}
