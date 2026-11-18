import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { serve, ready, click, gpuArgs } from './browser-helpers.mjs';

const server = await serve(new URL('./', import.meta.url).pathname);
const browser = await chromium.launch({ headless: true, args: gpuArgs });
try {
  const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  await page.addInitScript(() => {
    const input = { id: 'lesson-in', name: 'Simulated piano', state: 'connected', onmidimessage: null };
    window.fakeMidi = { inputs: new Map([[input.id, input]]), outputs: new Map(), onstatechange: null };
    Object.defineProperty(navigator, 'requestMIDIAccess', { value: async () => window.fakeMidi });
    Object.defineProperty(navigator, 'permissions', {
      configurable: true, value: { query: async () => ({ state: 'granted' }) },
    });
  });
  await ready(page, server.url);
  await click(page, 'settings'); await click(page, 'midi');
  await page.waitForFunction(() => fakeMidi.inputs.get('lesson-in').onmidimessage);
  await page.keyboard.press('Escape');
  await click(page, 'train'); await click(page, 'learn'); await click(page, 'lesson-home');
  await page.waitForFunction(() => tangentDiagnostics().mode === 'Lesson' &&
    tangentDiagnostics().lesson_id === 'find-home');
  async function note(key, down) {
    await page.evaluate(({ key, down }) => fakeMidi.inputs.get('lesson-in').onmidimessage({
      data: [down ? 0x90 : 0x80, key, down ? 90 : 0],
    }), { key, down });
    await page.waitForFunction(({ key, down }) =>
      tangentDiagnostics().solid_keys.includes(key) === down, { key, down });
  }
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_complete, false);
  await click(page, 'lesson-listen');
  await page.waitForFunction(() => tangentDiagnostics().lesson_audition.length > 0);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_cursor, 0,
    'The reference may never complete an exercise');
  await note(60, true);
  await page.waitForFunction(() => tangentDiagnostics().lesson_complete);
  await note(60, false); await click(page, 'lesson-next');
  await page.waitForFunction(() => tangentDiagnostics().lesson_step === 1);

  for (const key of [60, 64, 67]) await note(key, true);
  await page.waitForFunction(() => tangentDiagnostics().lesson_complete);
  for (const key of [60, 64, 67]) await note(key, false);
  await click(page, 'lesson-next');
  await page.waitForFunction(() => tangentDiagnostics().lesson_step === 2);
  await note(72, true); await note(72, false);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_cursor, 0,
    'An out-of-order note cannot advance');
  for (const key of [71, 72]) { await note(key, true); await note(key, false); }
  await page.waitForFunction(() => tangentDiagnostics().lesson_complete);
  await click(page, 'lesson-next');
  await page.waitForFunction(() => tangentDiagnostics().lesson_step === 3);
  await click(page, 'lesson-context');
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_context, 1);
  await click(page, 'lesson-context');
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_context, 2);
  await click(page, 'lesson-reveal');
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_reveal, true);
  await note(71, true); await note(71, false);
  await click(page, 'lesson-next');
  await page.waitForFunction(() => tangentDiagnostics().lesson_step === 4);

  await click(page, 'lesson-exit');
  await page.waitForFunction(() => tangentDiagnostics().screen === 'Learn');
  await click(page, 'lesson-major');
  await page.waitForFunction(() => tangentDiagnostics().lesson_id === 'major');
  await click(page, 'lesson-next');
  await page.waitForFunction(() => tangentDiagnostics().lesson_step === 1);
  await click(page, 'lesson-exit');
  await click(page, 'lesson-minor');
  await page.waitForFunction(() => tangentDiagnostics().lesson_id === 'minor');
  await click(page, 'lesson-exit');
  await click(page, 'lesson-fifths');
  await page.waitForFunction(() => tangentDiagnostics().lesson_id === 'fifths');
  await click(page, 'lesson-context');
  assert.equal((await page.evaluate(() => tangentDiagnostics())).lesson_fifth, 1);

  for (const [width, height] of [[390, 700], [1280, 420]]) {
    await page.setViewportSize({ width, height });
    await page.waitForFunction(({ width, height }) =>
      tangentDiagnostics().width === width && tangentDiagnostics().height === height, { width, height });
    for (const b of (await page.evaluate(() => tangentDiagnostics())).buttons) {
      assert.ok(b.x >= 0 && b.y >= 0 && b.x + b.w <= width && b.y + b.h <= height,
        b.id + ' fits ' + width + 'x' + height);
    }
    assert.ok((await page.evaluate(() => tangentDiagnostics())).buttons.some(b => b.id === 'lesson-context'));
  }
  assert.deepEqual(errors, []);
  console.log('Interactive lessons: MIDI, reference isolation, four modules, hints and responsive Bevy controls passed.');
} finally {
  await browser.close();
  await server.close();
}
