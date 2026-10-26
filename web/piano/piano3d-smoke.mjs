import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { serve, ready, click, gpuArgs } from './browser-helpers.mjs';

const server = await serve(new URL('./', import.meta.url).pathname);
const browser = await chromium.launch({ headless: true, args: gpuArgs });
const capture = process.env.TANGENT_CAPTURE_3D === '1';
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 } });
  await page.route('**/favicon.ico', route => route.fulfill({ status: 204, body: '' }));
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error' || m.text().includes('%cERROR%c')) errors.push(m.text()); });
  await ready(page, server.url);
  await click(page, 'start-play');
  await page.keyboard.press('ArrowRight');
  await page.waitForFunction(() => tangentDiagnostics().position === 5_000_000);
  await page.waitForTimeout(1200);
  assert.equal(await page.evaluate(() => tangentDiagnostics().piano_renderer), 'bevy-pbr-3d');
  // A successful boot/input report alone cannot detect failed materials. Read
  // the real screenshot and reject the engine's diagnostic magenta surface.
  const shot = await page.screenshot();
  const magenta = await page.evaluate(async data => {
    const image = new Image(); image.src = 'data:image/png;base64,' + data;
    await image.decode();
    const canvas = new OffscreenCanvas(image.width, image.height);
    const context = canvas.getContext('2d'); context.drawImage(image,0,0);
    const pixels = context.getImageData(0,0,image.width,image.height).data;
    let bad = 0;
    for(let i=0;i<pixels.length;i+=4) if(pixels[i]>220 && pixels[i+1]<35 && pixels[i+2]>220) bad++;
    return bad;
  }, shot.toString('base64'));
  assert.ok(magenta < 100, `3D material fallback visible: ${magenta} magenta pixels`);
  if (capture) await page.screenshot({path: new URL('../../docs/screenshots/piano-3d.png', import.meta.url).pathname});
  // Every visible key remains playable in the projected geometry. Black backs
  // are deliberately tested above the white-key strike line.
  const expected = await page.evaluate(() => tangentDiagnostics().expected_glow);
  for (const note of [60, 61, 64]) {
    const key = await page.evaluate(note => tangentDiagnostics().key_rects.find(k => k.key === note), note);
    const y = note === 61 ? key.y + 2 : key.y + key.h * 0.9;
    await page.mouse.move(key.x + key.w / 2, y);
    await page.mouse.down();
    await page.waitForFunction(note => tangentDiagnostics().solid_keys.includes(note), note);
    await page.waitForTimeout(150);
    assert.deepEqual(await page.evaluate(() => tangentDiagnostics().expected_glow), expected);
    if (capture && note === 60) await page.screenshot({path: new URL('../../docs/screenshots/piano-3d-held.png', import.meta.url).pathname});
    await page.mouse.up();
    await page.waitForFunction(() => tangentDiagnostics().solid_keys.length === 0);
  }
  if (capture) {
    for (const theme of ['ember','paper']) {
      await page.keyboard.press('KeyB');
      await page.waitForTimeout(500);
      await page.screenshot({path:new URL(`../../docs/screenshots/piano-3d-${theme}.png`, import.meta.url).pathname});
    }
    await page.keyboard.press('KeyB');
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
