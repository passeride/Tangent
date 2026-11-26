// GitHub Actions shards: run each piano script once, on separate runners.
// Keep npm test as the local developer command; CI verifies exact coverage.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(fileURLToPath(import.meta.url));
const shards = {
  foundations: [
    'playback-clock.test.mjs', 'progress-schema.test.mjs',
    'midi-output.test.mjs', 'storage-schema.test.mjs',
    'fingerprint-schema.test.mjs', 'library-schema.test.mjs',
    'folder-library.test.mjs', 'metadata-schema.test.mjs',
    'metadata-http.test.mjs', 'smoke.mjs', 'storage-smoke.mjs',
    'library-smoke.mjs', 'folder-smoke.mjs', 'metadata-smoke.mjs',
    'practice-midi-smoke.mjs', 'guided-smoke.mjs',
  ],
  practice: [
    'progress-smoke.mjs', 'speed-smoke.mjs', 'passage-smoke.mjs',
    'hud-smoke.mjs', 'goal-smoke.mjs',
  ],
  interface: [
    'bindings-smoke.mjs', 'personalization-smoke.mjs',
    'piano3d-smoke.mjs', 'explorer-smoke.mjs',
    'notation-layers-smoke.mjs', 'beaming-smoke.mjs',
    'lessons-smoke.mjs',
  ],
  sequences: ['flow-smoke.mjs', 'scale-smoke.mjs'],
};
const declared = JSON.parse(readFileSync(join(root, 'package.json'), 'utf8'))
  .scripts.test.split(/\s*&&\s*/).map(command => {
    const match = /^node ([\w.-]+\.mjs)$/.exec(command);
    assert.ok(match, 'Unsupported npm test command: ' + command);
    return match[1];
  });
const assigned = Object.values(shards).flat();
assert.equal(new Set(assigned).size, assigned.length, 'Duplicate CI test assignment');
assert.deepEqual([...assigned].sort(), [...declared].sort(),
  'CI shard coverage must exactly match npm test (add new tests to a shard)');

const shard = process.argv[2];
if (!Object.hasOwn(shards, shard)) {
  console.error('Expected shard: ' + Object.keys(shards).join(', '));
  process.exit(2);
}
console.log('Running shard ' + shard + ' (' + shards[shard].length + ' tests)');
const started = performance.now();
for (const file of shards[shard]) {
  console.log('::group::' + file);
  const begin = performance.now();
  const result = spawnSync(process.execPath, [file], {
    cwd: root,
    stdio: 'inherit',
    timeout: 9 * 60_000,
  });
  const seconds = ((performance.now() - begin) / 1000).toFixed(1);
  console.log('::endgroup::');
  if (result.error || result.status !== 0) {
    console.error(file + ' failed or timed out after ' + seconds + 's',
      result.error?.message ?? 'exit code ' + result.status);
    process.exit(result.status || 1);
  }
  console.log('PASS ' + file + ' (' + seconds + 's)');
}
console.log('Shard ' + shard + ' passed in ' +
  ((performance.now() - started) / 1000).toFixed(1) + 's');
