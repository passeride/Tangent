#!/usr/bin/env python3
"""Exercise the actual compiled launcher on each native CI runner."""
import os
import queue
import re
import subprocess
import sys
import threading
import urllib.request
process = subprocess.Popen([sys.argv[1], '--no-open'], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
lines = queue.Queue()
threading.Thread(target=lambda: lines.put(process.stdout.readline()), daemon=True).start()
try:
    line = lines.get(timeout=15)
    match = re.search(r'http://127\.0\.0\.1:\d+/', line)
    assert match, line
    url = match[0]
    expected_port = int(os.environ.get("TANGENT_PORT", "38765"))
    if expected_port:
        assert url == f"http://127.0.0.1:{expected_port}/", url
    with urllib.request.urlopen(url, timeout=10) as response:
        assert b'Tangent' in response.read()
    with urllib.request.urlopen(url+'pkg/tangent_player_bg.wasm', timeout=10) as response:
        assert response.headers['Content-Type'] == 'application/wasm'
        assert response.read(4) == b'\0asm'
    with urllib.request.urlopen(url+'vendor/sql-wasm.wasm', timeout=10) as response:
        assert response.headers['Content-Type'] == 'application/wasm'
        assert response.read(4) == b'\0asm'
    with urllib.request.urlopen(url+'vendor/sql.js-LICENSE.txt', timeout=10) as response:
        assert b'MIT license' in response.read()
    with urllib.request.urlopen(url+'samples/first-light.musicxml', timeout=10) as response:
        assert b'score-partwise' in response.read()
    with urllib.request.urlopen(url+'samples/ATTRIBUTION.md', timeout=10) as response:
        assert b'CC0 1.0' in response.read()
    print('Desktop launcher passed: loopback HTTP, player, real WASM, SQLite runtime, MusicXML and credits.')
finally:
    process.terminate()
    process.communicate(timeout=10)
