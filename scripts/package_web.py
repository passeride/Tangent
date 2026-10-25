#!/usr/bin/env python3
"""One reproducible static distribution for Pages and desktop embedding."""
from pathlib import Path
import argparse
import shutil
root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--output', default='dist/piano')
args = parser.parse_args()
output = Path(args.output).resolve()
source = root / 'web/piano'
if not output.is_relative_to(root / "dist") or output == root / "dist":
    raise SystemExit("Output must be a subdirectory of this repository's dist/")
required = ['pkg/tangent_player.js', 'pkg/tangent_player_bg.wasm', 'samples/first-light.musicxml', 'samples/ATTRIBUTION.md']
for path in required:
    if not (source / path).is_file():
        raise SystemExit(f'Missing {path}. Build the WASM package first.')
if output.exists():
    shutil.rmtree(output)
output.mkdir(parents=True)
for pattern in ['*.html', '*.css', '*.js']:
    for path in source.glob(pattern):
        shutil.copy2(path, output / path.name)
for directory in ['pkg', 'samples']:
    shutil.copytree(source / directory, output / directory)
for filename in ['THIRD_PARTY.md', 'THIRD_PARTY_NOTICES.txt']:
    shutil.copy2(root / filename, output / filename)
shutil.copytree(root / 'licenses', output / 'licenses')
(output / '.nojekyll').touch()
print(output)
