[CmdletBinding()]
param([switch]$SelfTest, [switch]$Tests, [string]$TargetDirectory = "$env:LOCALAPPDATA\Temp\adb-on-target")
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$root = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
if (-not [Environment]::Is64BitOperatingSystem) { throw '64비트 Windows가 필요합니다.' }
$cargo = (Get-Command cargo.exe -ErrorAction Stop).Source
if ($SelfTest) {
  if (-not (Test-Path -LiteralPath (Join-Path $root 'Cargo.toml'))) { throw 'Cargo.toml이 없습니다.' }
  @{ status='ok'; platform='windows'; writes_release=$false; starts_gui=$false } | ConvertTo-Json -Compress
  exit 0
}
$target = [IO.Path]::GetFullPath($TargetDirectory)
if ($target.StartsWith($root, [StringComparison]::OrdinalIgnoreCase)) { throw '빌드 출력은 프로젝트 밖에 두세요.' }
[IO.Directory]::CreateDirectory($target) | Out-Null
$env:CARGO_TARGET_DIR = $target
$env:CARGO_BUILD_JOBS = '1'
$env:CARGO_INCREMENTAL = '0'
$env:SLINT_STYLE = 'fluent-light'
$log = Join-Path $target 'build.stdout.log'
$errors = Join-Path $target 'build.stderr.log'
# Dependency panic locations would otherwise embed the build machine's user profile path.
$cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE '.cargo' }
if (-not $Tests) { $env:RUSTFLAGS = "-C target-feature=+crt-static --remap-path-prefix=$cargoHome=/cargo --remap-path-prefix=$target=/target --remap-path-prefix=$root=/adb-on" }
$arguments = if ($Tests) { @('test','--no-default-features','--locked','--lib') } else { @('build','--release','--locked','--target','x86_64-pc-windows-msvc') }
$p = Start-Process -FilePath $cargo -ArgumentList $arguments -WorkingDirectory $root -RedirectStandardOutput $log -RedirectStandardError $errors -PassThru
# .Handle을 한 번 읽어 두어야 종료 후 ExitCode가 채워진다(PowerShell Start-Process 관용구). 값 자체는 쓰지 않는다.
$nativeHandle = $p.Handle
$deadline = [DateTime]::UtcNow.AddMinutes(30)
try {
  while (-not $p.WaitForExit(2000)) {
    $available = (Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory * 1KB
    if ($available -lt 2GB -or [DateTime]::UtcNow -ge $deadline) { throw '메모리 하한 또는 빌드 시간 상한에 도달했습니다.' }
  }
  Get-Content -LiteralPath $log -Encoding UTF8 -Tail 30
  Get-Content -LiteralPath $errors -Encoding UTF8 -Tail 35
  if ($p.ExitCode -ne 0) { throw "빌드 실패: $($p.ExitCode)" }
} finally {
  if (-not $p.HasExited) { & taskkill.exe /PID $p.Id /T /F | Out-Null; $p.WaitForExit() }
}
if (-not $Tests) {
  $dist = Join-Path $root 'dist\windows-x64'
  [IO.Directory]::CreateDirectory($dist) | Out-Null
  Copy-Item -LiteralPath (Join-Path $target 'x86_64-pc-windows-msvc\release\adb-on.exe') -Destination (Join-Path $dist 'adb-on.exe')
  # Guard: the shipped binary must not carry this machine's user profile path in any string table.
  $exeBytes = [IO.File]::ReadAllBytes((Join-Path $dist 'adb-on.exe'))
  $profileAscii = [Text.Encoding]::ASCII.GetBytes($env:USERPROFILE)
  $profileWide = [Text.Encoding]::Unicode.GetBytes($env:USERPROFILE)
  $latin1 = [Text.Encoding]::GetEncoding(28591) # byte-transparent; Windows PowerShell 5.1 has no Latin1 property
  $asText = $latin1.GetString($exeBytes)
  if ($asText.Contains($latin1.GetString($profileAscii)) -or $asText.Contains($latin1.GetString($profileWide))) { throw '실행 파일에 빌드 PC의 사용자 경로가 남아 있습니다.' }
  $diagnostic = Start-Process -FilePath (Join-Path $dist 'adb-on.exe') -ArgumentList '--self-test' -RedirectStandardOutput (Join-Path $target 'self-test.json') -PassThru
  $diagnosticHandle = $diagnostic.Handle # 위와 같은 ExitCode 확보 관용구
  if (-not $diagnostic.WaitForExit(30000)) { Stop-Process -Id $diagnostic.Id; throw '진단 시간 상한 초과' }
  if ($diagnostic.ExitCode -ne 0) { throw '배포 실행 파일 진단 실패' }
  $env:PYTHONUTF8 = '1'
  & python.exe (Join-Path $root 'scripts\export-licenses.py') (Join-Path $dist 'licenses')
  if ($LASTEXITCODE -ne 0) { throw '라이선스 고지 준비 실패' }
  Copy-Item -LiteralPath (Join-Path $root 'LICENSE.txt'),(Join-Path $root 'THIRD-PARTY-NOTICES.md') -Destination $dist
  $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $dist 'adb-on.exe')).Hash.ToLowerInvariant()
  [IO.File]::WriteAllText((Join-Path $dist 'SHA256SUMS.txt'), "$hash  adb-on.exe`n", [Text.UTF8Encoding]::new($false))
  $archive = Join-Path $root 'dist\adb-on-windows-x64.zip'
  & python.exe -m zipfile -c $archive (Join-Path $dist 'adb-on.exe') (Join-Path $dist 'LICENSE.txt') (Join-Path $dist 'THIRD-PARTY-NOTICES.md') (Join-Path $dist 'licenses') (Join-Path $dist 'SHA256SUMS.txt')
  if ($LASTEXITCODE -ne 0) { throw 'ZIP 생성 실패' }
  $archiveHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $archive).Hash.ToLowerInvariant()
  [IO.File]::WriteAllText("$archive.sha256", "$archiveHash  adb-on-windows-x64.zip`n", [Text.UTF8Encoding]::new($false))
  Write-Output "산출물: $archive"
}
