#!/usr/bin/env python3
"""Smoke the real platform binary without requiring an interactive GPU in CI."""
import subprocess
import sys

result = subprocess.run(
    [sys.argv[1], "--self-test"],
    check=True, capture_output=True, text=True, timeout=30,
)
assert "Tangent native player" in result.stdout, result.stdout
assert "embedded music OK" in result.stdout, result.stdout
print("Native desktop binary passed: executable startup and Rust music import.")
