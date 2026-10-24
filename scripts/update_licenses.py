#!/usr/bin/env python3
"""Regenerate notices from original registry packages, never from another app."""
import json
import pathlib
import subprocess
root = pathlib.Path(__file__).resolve().parents[1]
metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--format-version', '1', '--locked'], cwd=root))
packages = sorted((p for p in metadata['packages'] if p['source']), key=lambda p:(p['name'],p['version']))
lines = ['# Third-party dependencies', '', 'Generated from Cargo metadata/lockfiles on 2026-10-24. Includes resolved target/build dependencies; this is not a claim that each is linked into every target. The project-wide license remains unselected.', '', 'The web player uses Bevy 0.20 UI/wgpu with the WebGL2 backend. Audio remains Web Audio. Native launchers embed that same web distribution. The pure music/import crates remain independent of rendering. Bevy renderer dependencies are scoped to wasm32.', '', '| Package | Version | Declared license | Original source |', '| --- | --- | --- | --- |']
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
notices = ['Third-party registry license texts. Identical texts are deduplicated with all applicable packages listed.\n']
for text, credits in groups.items():
    notices.append('\n'+'='*72+'\nApplies to:\n'+'\n'.join(credits)+'\n'+'='*72+'\n'+text+'\n')
notices.append('\nBevy embedded Fira Mono subset: digitized data copyright © 2012–2015 The Mozilla Foundation and Telefonica S.A. SIL Open Font License 1.1. Full font license follows.\n'+(root/'licenses/FiraMono-OFL.txt').read_text())
lines += ['', 'Checksums: Cargo.lock. License texts: [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).', '', '## Registry archives without standalone license files', '']
lines += ['- '+item+' (manifest declaration; no standalone license file found).' for item in missing]
lines += ['', '## Font and MIDI assets', '', 'Bevy default_font embeds FiraMono-subset.ttf: digitized data copyright © 2012–2015 The Mozilla Foundation and Telefonica S.A.; SIL OFL 1.1. The complete [font license](licenses/FiraMono-OFL.txt) is included in all distributions and notices. Source: https://github.com/bevyengine/bevy/tree/v0.20.0/assets/fonts.', '', 'Bach Prelude BWV 924a: unmodified MIDI, typeset by jeff covey © 2007 / Mutopia, CC BY-SA 3.0. [Source, hash and attribution](web/piano/samples/ATTRIBUTION.md); full license ships alongside the MIDI. No SoundFont or sampled piano is bundled.', '', '## Test tooling', '', 'Playwright/playwright-core 1.56.1 (Apache-2.0) and optional fsevents 2.3.2 (MIT) are locked in the web package-lock.json files. They are development tools, not shipped application dependencies. Chromium is downloaded for tests. Sources: https://github.com/microsoft/playwright and https://github.com/fsevents/fsevents.', '', 'Regenerate with `python scripts/update_licenses.py` after `cargo fetch`. Preserve the separate font/music licenses. This inventory is not a legal opinion or a clean-room certification.']
(root/'THIRD_PARTY.md').write_text('\n'.join(lines)+'\n')
(root/'THIRD_PARTY_NOTICES.txt').write_text('\n'.join(line.rstrip() for line in ''.join(notices).splitlines()).rstrip()+'\n')
print(f'{len(packages)} packages; {len(groups)} unique license texts; {len(missing)} missing standalone files')
