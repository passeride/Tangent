import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { serve, ready, click, gpuArgs } from './browser-helpers.mjs';

const server = await serve(new URL('./', import.meta.url).pathname);
const browser = await chromium.launch({ headless: true, args: gpuArgs });
const capture = process.env.TANGENT_CAPTURE_3D === '1';
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 } });
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });
  await ready(page, server.url);
  await click(page, 'start-play');
  await page.keyboard.press('ArrowRight');
  await page.waitForFunction(() => tangentDiagnostics().position === 5_000_000);
  await page.waitForTimeout(1200);
  assert.equal(await page.evaluate(() => tangentDiagnostics().piano_renderer), 'bevy-pbr-3d');
  if (capture) await page.screenshot({path: new URL('../../docs/screenshots/piano-3d.png', import.meta.url).pathname});
  // Every visible key remains playable in the projected geometry. Black backs
  // are deliberately tested above the white-key strike line.
  for (const note of [60, 61, 64]) {
    const key = await page.evaluate(note => tangentDiagnostics().key_rects.find(k => k.key === note), note);
    const y = note === 61 ? key.y + 2 : key.y + key.h * 0.9;
    await page.mouse.move(key.x + key.w / 2, y);
    await page.mouse.down();
    await page.waitForFunction(note => tangentDiagnostics().solid_keys.includes(note), note);
    await page.waitForTimeout(150);
    if (capture && note === 60) await page.screenshot({path: new URL('../../docs/screenshots/piano-3d-held.png', import.meta.url).pathname});
    await page.mouse.up();
    await page.waitForFunction(() => tangentDiagnostics().solid_keys.length === 0);
  }
  // Preserve the piano through resize and then cleanly leave its screen.
  for (const [width, height] of [[390,700], [1280,420], [3840,2160]]) {
    await page.setViewportSize({width,height});
    await page.waitForFunction(width => Math.abs(tangentDiagnostics().width - width) < 2, width);
    await page.waitForTimeout(400);
    const key = await page.evaluate(() => tangentDiagnostics().key_rects.find(k => k.key === 61));
    await page.mouse.move(key.x + key.w / 2, key.y + 2);
    await page.mouse.down();
    await page.waitForFunction(() => tangentDiagnostics().solid_keys.includes(61));
    await page.mouse.up();
    await page.waitForFunction(() => tangentDiagnostics().solid_keys.length === 0);
  }
  await page.setViewportSize({width:1440,height:960});
  await page.keyboard.press('Escape');
  await click(page, 'main');
  await page.waitForFunction(() => tangentDiagnostics().screen === 'Main');
  await click(page, 'freeform');
  await page.waitForFunction(() => tangentDiagnostics().mode === 'FreeForm');
  assert.equal(await page.evaluate(() => tangentDiagnostics().expected_glow.length), 0);
  assert.deepEqual(errors, []);
  console.log('3D piano: real GPU boot, raised black/white pointer input, release, resize and scene return passed.');
} finally {
  await browser.close();
  await server.close();
}
