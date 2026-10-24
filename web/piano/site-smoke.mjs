import assert from "node:assert/strict";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const server = await serve(process.argv[2] ?? "dist/piano", "/Tangent/");
let browser;
try {
  browser = await chromium.launch({ headless: true, args: gpuArgs });
  const page = await browser.newPage(),
    errors = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await ready(page, server.url);
  await click(page, "sample");
  await page.waitForFunction(() => tangentDiagnostics().notes === 228);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().position > 300_000);
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  assert.deepEqual(errors, []);
  console.log(
    "Packaged Bevy Pages site passed: /Tangent/ paths, GPU UI, real WASM, CC MIDI and sound.",
  );
} finally {
  await browser?.close();
  await server.close();
}
