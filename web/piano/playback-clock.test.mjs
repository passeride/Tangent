import assert from 'node:assert/strict';
import { PlaybackClock } from './playback-clock.js';
for (const percent of [25, 60, 70, 100, 120, 200]) {
  const c = new PlaybackClock(); c.reset(10, 2_000_000, percent);
  assert.ok(Math.abs(c.position(11) - (2_000_000 + percent * 10_000)) < 1e-6);
  assert.ok(Math.abs(c.time(2_000_000 + percent * 10_000) - 11) < 1e-9);
  assert.ok(Math.abs(c.position(11, 0.1) - c.position(11) - percent * 1000) < 1e-6);
  // Reanchor at the same score position for a live tempo change, without a jump.
  const pos = c.position(11); c.reset(11, pos, 120);
  assert.ok(Math.abs(c.position(11) - pos) < 1e-6);
}
console.log('Playback clock: tempo, event deadlines, lookahead and reanchoring passed.');
