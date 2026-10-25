import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const server = await serve(new URL("./", import.meta.url).pathname);
const browser = await chromium.launch({ headless: true, args: gpuArgs });
try {
  const page = await browser.newPage({ viewport: { width: 1100, height: 800 } });
  await ready(page, server.url);
  await click(page, "library");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Library");
  await click(page, "import");
  const buffer = await readFile(new URL("samples/first-light.musicxml", import.meta.url));
  await page.locator("#midi-file").setInputFiles({ name: "library-fixture.musicxml", mimeType: "application/xml", buffer });
  await page.waitForFunction(() => tangentDiagnostics().buttons.some(b => b.id === "library-open"));
  await page.reload(); await ready(page, server.url);
  await click(page, "library");
  await click(page, "library-open");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session" && tangentDiagnostics().title === "First light");
  await click(page, "help"); await click(page, "main"); await click(page, "library");
  await click(page, "library-remove");
  // First click only asks for confirmation; the piece still exists.
  assert(await page.evaluate(() => tangentDiagnostics().buttons.some(b => b.id === "library-open")));
  await click(page, "library-remove");
  await page.waitForFunction(() => !tangentDiagnostics().buttons.some(b => b.id === "library-open"));
  console.log("Library browser passed: import, durable reload, reopen and confirmed deletion in Bevy UI.");
} finally { await browser.close(); await server.close(); }
