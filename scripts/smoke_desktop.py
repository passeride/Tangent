#!/usr/bin/env python3
"""Exercise the actual compiled launcher on each native CI runner."""
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
    with urllib.request.urlopen(url, timeout=10) as response:
        assert b'Tangent' in response.read()
    with urllib.request.urlopen(url+'pkg/tangent_player_bg.wasm', timeout=10) as response:
        assert response.headers['Content-Type'] == 'application/wasm'
        assert response.read(4) == b'\0asm'
    with urllib.request.urlopen(url+'samples/bach-prelude-bwv924a.mid', timeout=10) as response:
        assert response.read(4) == b'MThd'
    with urllib.request.urlopen(url+'samples/ATTRIBUTION.md', timeout=10) as response:
        assert b'CC BY-SA 3.0' in response.read()
    print('Desktop launcher passed: loopback HTTP, player, real WASM, MIDI and credits.')
finally:
    process.terminate()
    process.communicate(timeout=10)
