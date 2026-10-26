#!/usr/bin/env python3
"""Generate Tangent's original circle-and-tangent Windows icon with stdlib only."""
import argparse
import math
from pathlib import Path
import struct
import zlib

SIZES = (16, 32, 48, 256)
BG = (11, 23, 32)
RING = (140, 224, 193)
STROKE = (234, 242, 238)


def rgba_at(x: float, y: float) -> tuple[int, int, int, int]:
    # Same dark/sea-glass palette and ring/tangent identity as the Bevy menu.
    dx, dy = x - 0.51, y - 0.50
    circle = math.hypot(dx, dy)
    ring = abs(circle - 0.275) < 0.038
    ax, ay, bx, by = 0.24, 0.78, 0.78, 0.24
    vx, vy = bx - ax, by - ay
    t = max(0.0, min(1.0, ((x - ax) * vx + (y - ay) * vy) / (vx * vx + vy * vy)))
    slash = math.hypot(x - (ax + t * vx), y - (ay + t * vy)) < 0.029
    color = STROKE if slash else RING if ring else BG
    return (*color, 255)


def png(size: int) -> bytes:
    rows = bytearray()
    # 3x supersampling keeps small taskbar and Explorer icons crisp.
    for y in range(size):
        rows.append(0)
        for x in range(size):
            channel = [0, 0, 0, 0]
            for oy in range(3):
                for ox in range(3):
                    sample = rgba_at((x + (ox + 0.5) / 3) / size,
                                     (y + (oy + 0.5) / 3) / size)
                    for c in range(4):
                        channel[c] += sample[c]
            rows.extend(v // 9 for v in channel)

    def chunk(kind: bytes, payload: bytes) -> bytes:
        return (struct.pack('>I', len(payload)) + kind + payload +
                struct.pack('>I', zlib.crc32(kind + payload) & 0xffffffff))

    return (b'\x89PNG\r\n\x1a\n' +
            chunk(b'IHDR', struct.pack('>IIBBBBB', size, size, 8, 6, 0, 0, 0)) +
            chunk(b'IDAT', zlib.compress(bytes(rows), level=9)) +
            chunk(b'IEND', b''))


def icon_bytes() -> bytes:
    images = [png(size) for size in SIZES]
    header = struct.pack('<HHH', 0, 1, len(images))
    offset = len(header) + 16 * len(images)
    entries = bytearray()
    for size, data in zip(SIZES, images):
        entries.extend(struct.pack('<BBBBHHII', size % 256, size % 256,
                                   0, 0, 1, 32, len(data), offset))
        offset += len(data)
    return header + entries + b''.join(images)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(icon_bytes())
    print(f'Tangent icon: {args.output}')


if __name__ == '__main__':
    main()
