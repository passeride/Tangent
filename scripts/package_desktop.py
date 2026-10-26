#!/usr/bin/env python3
"""Stage only launchable desktop files for GitHub's single artifact ZIP layer."""
import argparse
from pathlib import Path
import plistlib
import shutil

parser = argparse.ArgumentParser()
parser.add_argument('--target', required=True)
parser.add_argument('--binary', required=True, type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
output = root / 'dist' / f'Tangent-{args.target}'
if output.exists():
    shutil.rmtree(output)
output.mkdir(parents=True)

if 'apple-darwin' in args.target:
    app = output / 'Tangent.app' / 'Contents'
    executable = app / 'MacOS' / 'Tangent'
    executable.parent.mkdir(parents=True)
    (app / 'Info.plist').write_bytes(plistlib.dumps({
        'CFBundleName': 'Tangent', 'CFBundleDisplayName': 'Tangent',
        'CFBundleIdentifier': 'com.passeride.tangent',
        'CFBundleExecutable': 'Tangent', 'CFBundlePackageType': 'APPL',
        'CFBundleVersion': '0.1.0', 'CFBundleShortVersionString': '0.1.0',
    }))
elif 'windows' in args.target:
    executable = output / 'Tangent.exe'
else:
    executable = output / 'tangent'

shutil.copy2(args.binary, executable)
executable.chmod(0o755)

# Keep developer/test scripts and all source/docs out of end-user artifacts.
expected = ({'Tangent.app/Contents/Info.plist',
             'Tangent.app/Contents/MacOS/Tangent'}
            if 'apple-darwin' in args.target else {executable.name})
actual = {p.relative_to(output).as_posix() for p in output.rglob('*') if p.is_file()}
assert actual == expected, f'Unexpected release files: {actual ^ expected}'
print(f'Staged {output}: {", ".join(sorted(actual))}')
