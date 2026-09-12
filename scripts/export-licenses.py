"""잠긴 대상 플랫폼 의존성의 고지 원문을 배포 폴더에 모은다."""
import json
import pathlib
import shutil
import subprocess
import sys

output = pathlib.Path(sys.argv[1]).resolve()
host = subprocess.check_output(['rustc', '-vV'], text=True, encoding="utf-8").split('host: ')[1].splitlines()[0]
metadata = json.loads(subprocess.check_output([
    'cargo', 'metadata', '--locked', '--offline', '--format-version', '1',
    '--filter-platform', host,
], text=True, encoding="utf-8"))
runtime = subprocess.check_output([
    'cargo', 'tree', '--locked', '--offline', '--edges', 'normal,no-proc-macro',
    '--prefix', 'none', '--format', '{p}',
], text=True, encoding='utf-8')
selected = {tuple(line.split()[:2]) for line in runtime.splitlines() if line.strip()}
output.mkdir(parents=True, exist_ok=True)
index = ['# 포함된 구성요소 고지', '', '대상 플랫폼 런타임 의존성의 고지입니다.',
         'Slint 구성요소는 Royalty-free 대안을 선택합니다. 각 원문의 별도 권리를 보존합니다.', '']
missing = []
for package in sorted(metadata['packages'], key=lambda p: (p['name'], p['version'])):
    if package['source'] is None or (package['name'], 'v' + package['version']) not in selected:
        continue
    name = package['name'] + '-' + package['version']
    root = pathlib.Path(package['manifest_path']).parent
    files = []
    for child in root.iterdir():
        if child.name.lower().startswith(('license', 'licence', 'notice', 'copying')):
            files.extend([child] if child.is_file() else [p for p in child.rglob('*') if p.is_file()])
    if package.get('license_file'):
        files.append(root / package['license_file'])
    index.extend([f"## {name}", '', f"라이선스: {package.get('license', '동봉 원문 참조')}",
                  f"원본: https://crates.io/crates/{package['name']}/{package['version']}", ''])
    supplement = pathlib.Path(__file__).parent / 'licenses' / name
    if not files and supplement.is_dir():
        shutil.copytree(supplement, output / name, dirs_exist_ok=True)
    elif not files:
        missing.append(name)
    for source in set(files):
        target = output / name / source.relative_to(root)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    if package.get('license') == 'MPL-2.0':
        # 변경하지 않은 MPL 구성요소의 정확한 소스도 동봉한다. 앱 소스는 포함하지 않는다.
        cache = root.parent.parent.parent / 'cache' / root.parent.name / (name + '.crate')
        if not cache.is_file():
            raise SystemExit(f'MPL 원본 압축 파일 없음: {name}')
        shutil.copyfile(cache, output / (name + '.crate'))
        index.append(f'변경 없는 MPL 소스: {name}.crate (gzip tar), MPL-2.0으로 제공됩니다.\n')
toolchain = pathlib.Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True, encoding='utf-8').strip())
rust_docs = toolchain / 'share' / 'doc' / 'rust'
if not (rust_docs / 'COPYRIGHT-library.html').is_file():
    raise SystemExit('Rust 표준 라이브러리 저작권 고지가 없습니다.')
shutil.copyfile(rust_docs / 'COPYRIGHT-library.html', output / 'RUST-COPYRIGHT-library.html')
shutil.copytree(rust_docs / 'licenses', output / 'rust-licenses', dirs_exist_ok=True)
index.extend(['## Rust 표준 라이브러리', '', 'RUST-COPYRIGHT-library.html과 rust-licenses/를 참조하세요.', ''])
(output / 'INDEX.md').write_text('\n'.join(index), encoding='utf-8')
if missing:
    raise SystemExit('원문 누락 확인 필요: ' + ', '.join(missing))
print(f'라이선스 고지 준비: {output}')
