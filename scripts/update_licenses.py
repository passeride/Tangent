#!/usr/bin/env python3
"""Regenerate notices from original dependency packages, never from another app."""
import json
import pathlib
import subprocess
root = pathlib.Path(__file__).resolve().parents[1]
metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--format-version', '1', '--locked'], cwd=root))
packages = sorted((p for p in metadata['packages'] if p['source']), key=lambda p:(p['name'],p['version']))
lines = ['# Third-party dependencies', '', 'Generated from Cargo metadata/lockfiles on 2026-10-25. Includes resolved target/build dependencies; this is not a claim that each is linked into every target. The project-wide license remains unselected.', '', 'The shared Bevy 0.20 UI uses WebGL2/Web Audio in the browser and native wgpu/audio/MIDI on desktop. Native GPU effects use the pinned bevy_hanabi source commit in Cargo.lock; web uses bounded Bevy UI effects. The pure music/import crates remain independent of rendering.', '', '| Package | Version | Declared license | Original source |', '| --- | --- | --- | --- |']
groups = {}
missing = []
for p in packages:
    license = p['license'] or 'See package license file'
    source = p['repository'] or p['homepage'] or 'https://crates.io/crates/'+p['name']
    lines.append(f"| {p['name']} | {p['version']} | {license} | {source} |")
    directory = pathlib.Path(p['manifest_path']).parent
    paths = sorted(f for f in directory.rglob('*') if f.is_file() and (f.name.upper().startswith(('LICENSE','LICENCE','COPYING','COPYRIGHT','UNLICENSE')) or (p.get('license_file') and f == directory/p['license_file'])))
    if not paths: missing.append(f"{p['name']} {p['version']}: {license}")
    for path in paths:
        try: text=path.read_text()
        except UnicodeDecodeError: continue
        groups.setdefault(text, []).append(f"{p['name']} {p['version']} — {path.relative_to(directory)} ({license})")
notices = ['Third-party dependency license texts. Identical texts are deduplicated with all applicable packages listed.\n']
for text, credits in groups.items():
    notices.append('\n'+'='*72+'\nApplies to:\n'+'\n'.join(credits)+'\n'+'='*72+'\n'+text+'\n')
notices.append('\nBevy embedded Fira Mono subset: digitized data copyright © 2012–2015 The Mozilla Foundation and Telefonica S.A. SIL Open Font License 1.1. Full font license follows.\n'+(root/'licenses/FiraMono-OFL.txt').read_text())
notices.append('\nBravura music font, Steinberg Media Technologies GmbH. SIL OFL 1.1. Full license follows.\n'+(root/'licenses/Bravura-OFL.txt').read_text())
lines += ['', 'Checksums: Cargo.lock. License texts: [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).', '', '## Registry archives without standalone license files', '']
lines += ['- '+item+' (manifest declaration; no standalone license file found).' for item in missing]
lines += ['', '## Font and music assets', '', 'Bevy default_font embeds FiraMono-subset.ttf: digitized data copyright © 2012–2015 The Mozilla Foundation and Telefonica S.A.; SIL OFL 1.1. The complete [font license](licenses/FiraMono-OFL.txt) is included in all distributions and notices. Source: https://github.com/bevyengine/bevy/tree/v0.20.0/assets/fonts.', '', 'Bravura: unmodified font by Steinberg Media Technologies GmbH, SIL OFL 1.1; [full license](licenses/Bravura-OFL.txt). Source and pinned commit: [asset register](docs/design/assets/README.md). First light: original MusicXML piano study, CC0 1.0; [provenance](web/piano/samples/ATTRIBUTION.md). No SoundFont or sampled piano is bundled.', '', '## Test tooling', '', 'Playwright/playwright-core 1.56.1 (Apache-2.0) and optional fsevents 2.3.2 (MIT) are locked in the web package-lock.json files. They are development tools, not shipped application dependencies. Chromium is downloaded for tests. Sources: https://github.com/microsoft/playwright and https://github.com/fsevents/fsevents.', '', 'Regenerate with `python scripts/update_licenses.py` after `cargo fetch`. Preserve the separate font/music licenses. This inventory is not a legal opinion or a clean-room certification.']
sql_license = (root/'licenses/sql.js-MIT.txt').read_text()
lines += ['', '## SQLite browser runtime', '', 'sql.js 1.15.1 (MIT), SQLite compiled to WebAssembly. Locked in web/piano/package-lock.json; source https://github.com/sql-js/sql.js. SQLite is public domain. npm ci copies the exact runtime and complete upstream license into vendor/ for offline packaging. Full license: [sql.js-MIT.txt](licenses/sql.js-MIT.txt).']
notices.append('\nsql.js 1.15.1 — SQLite WebAssembly runtime\n'+sql_license+'\n')
(root/'THIRD_PARTY.md').write_text('\n'.join(lines)+'\n')
(root/'THIRD_PARTY_NOTICES.txt').write_text('\n'.join(line.rstrip() for line in ''.join(notices).splitlines()).rstrip()+'\n')
print(f'{len(packages)} packages; {len(groups)} unique license texts; {len(missing)} missing standalone files')
