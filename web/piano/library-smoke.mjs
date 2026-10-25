import assert from "node:assert/strict";
import { readFile, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const server = await serve(new URL("./", import.meta.url).pathname);
const profile = await mkdtemp(join(tmpdir(), "tangent-library-"));
let context;
try {
  context = await chromium.launchPersistentContext(profile, { headless: true, args: gpuArgs, viewport: { width: 1100, height: 800 } });
  let page = await context.newPage();
  await ready(page, server.url);
  await click(page, "library");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Library");
  await click(page, "import");
  const buffer = await readFile(new URL("samples/first-light.musicxml", import.meta.url));
  await page.locator("#midi-file").setInputFiles({ name: "library-fixture.musicxml", mimeType: "application/xml", buffer });
  await page.waitForFunction(() => tangentDiagnostics().buttons.some(b => b.id === "library-open"));
  await context.close();
  context = await chromium.launchPersistentContext(profile, { headless: true, args: gpuArgs, viewport: { width: 1100, height: 800 } });
  page = await context.newPage();
  await ready(page, server.url);
  await click(page, "library");
  await click(page, "library-open");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session" && tangentDiagnostics().title === "First light" && tangentDiagnostics().status.includes("notes loaded"));
  await click(page, "help"); await click(page, "main"); await click(page, "library");
  await click(page, "library-remove");
  // First click only asks for confirmation; the piece still exists.
  assert(await page.evaluate(() => tangentDiagnostics().buttons.some(b => b.id === "library-open")));
  await click(page, "library-remove");
  await page.waitForFunction(() => !tangentDiagnostics().buttons.some(b => b.id === "library-open"));
  console.log("Library browser passed: import, durable reload, reopen and confirmed deletion in Bevy UI.");
} finally { await context?.close(); await server.close(); await rm(profile, { recursive: true, force: true }); }
