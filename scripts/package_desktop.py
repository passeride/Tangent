#!/usr/bin/env python3
"""Package the embedded local player; no web runtime needs installing."""
from pathlib import Path
import argparse
import hashlib
import plistlib
import shutil
import tarfile
import zipfile
parser = argparse.ArgumentParser()
parser.add_argument('--target', required=True)
parser.add_argument('--binary', required=True, type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
name = f'Tangent-{args.target}'
output = root / 'dist' / name
if output.exists(): shutil.rmtree(output)
output.mkdir(parents=True)
if 'apple-darwin' in args.target:
    app = output / 'Tangent.app' / 'Contents'
    executable = app / 'MacOS' / 'Tangent'
    executable.parent.mkdir(parents=True)
    (app / 'Info.plist').write_bytes(plistlib.dumps({
        'CFBundleName':'Tangent', 'CFBundleDisplayName':'Tangent',
        'CFBundleIdentifier':'com.passeride.tangent', 'CFBundleExecutable':'Tangent',
        'CFBundlePackageType':'APPL', 'CFBundleVersion':'0.1.0',
        'CFBundleShortVersionString':'0.1.0',
    }))
else:
    executable = output / ('Tangent.exe' if 'windows' in args.target else 'tangent')
shutil.copy2(args.binary, executable)
executable.chmod(0o755)
shutil.copy2(root / 'docs/distribution.md', output / 'README.md')
shutil.copy2(root / 'docs/test-p45.md', output / 'TEST-P45.md')
shutil.copy2(root / 'docs/practice-progress.md', output / 'PRACTICE-PROGRESS.md')
(output / 'adr').mkdir(exist_ok=True)
shutil.copy2(root / 'docs/adr/0018-scored-sessions-and-adaptive-review.md', output / 'adr/0018-scored-sessions-and-adaptive-review.md')
if 'windows' in args.target:
    shutil.copy2(root / 'scripts/Test-Tangent.ps1', output / 'Test-Tangent.ps1')
shutil.copytree(root / 'web/piano/samples', output / 'samples')
for filename in ['THIRD_PARTY.md', 'THIRD_PARTY_NOTICES.txt']:
    shutil.copy2(root / filename, output / filename)
archive = root / 'dist' / (name + ('.tar.gz' if 'linux' in args.target else '.zip'))
if 'linux' in args.target:
    with tarfile.open(archive, 'w:gz') as stream: stream.add(output, arcname=name)
else:
    with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as stream:
        for file in sorted(output.rglob('*')):
            if file.is_file(): stream.write(file, file.relative_to(output.parent))
archive.with_name(archive.name+'.sha256').write_text(hashlib.sha256(archive.read_bytes()).hexdigest()+'  '+archive.name+'\n')
print(archive)
