import assert from "node:assert/strict";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const server = await serve(new URL("./", import.meta.url).pathname);
let browser;
try {
  browser = await chromium.launch({ headless: true, args: gpuArgs });
  const page = await browser.newPage({
      viewport: { width: 1440, height: 960 },
    }),
    errors = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await ready(page, server.url);
  assert.equal(await page.locator("canvas").count(), 1);
  assert.equal(await page.locator("button,select").count(), 0);
  assert.equal(await page.locator("#startup").isHidden(), true);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).notes, 130);
  await page.keyboard.down("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().live.includes(60));
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  await page.keyboard.up("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().live.length === 0);
  const key = await page.evaluate(() =>
    tangentDiagnostics().key_rects.find((k) => k.key === 61),
  );
  await page.mouse.move(key.x + key.w / 2, key.y + 10);
  await page.mouse.down();
  await page.waitForFunction(() => tangentDiagnostics().live.includes(61));
  await page.mouse.up();
  await page.waitForFunction(() => tangentDiagnostics().live.length === 0);
  await page.keyboard.press("ArrowUp");
  await page.waitForFunction(() => tangentDiagnostics().volume > 0.49);
  await page.keyboard.press("ArrowDown");
  await page.waitForFunction(() => tangentDiagnostics().volume < 0.46);
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => tangentDiagnostics().position === 5_000_000);
  await page.keyboard.press("Shift+ArrowRight");
  await page.waitForFunction(
    () => tangentDiagnostics().position === 20_000_000,
  );
  await page.keyboard.press("Home");
  await page.waitForFunction(() => tangentDiagnostics().position === 0);
  for (const view of ["Staff", "Notes", "Score"]) {
    await page.keyboard.press("KeyV");
    await page.waitForFunction(
      (view) => tangentDiagnostics().view === view,
      view,
    );
    const notation = await page.evaluate(() => tangentDiagnostics());
    assert.equal(notation.staff_lines,10);
    assert.equal(notation.music_clefs,2);
    assert.equal(notation.written_measures,16);
    assert.equal(notation.measure_visits,23);
    assert.equal(notation.ending_measures,6);
  }
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => tangentDiagnostics().position === 5_000_000);
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => tangentDiagnostics().position === 10_000_000);
  await page.waitForFunction(() => tangentDiagnostics().ending_labels === 2);
  await page.keyboard.press("Home");
  await page.waitForFunction(() => tangentDiagnostics().position === 0);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing);
  await page.waitForFunction(() => tangentDiagnostics().position > 500_000);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => !tangentDiagnostics().playing);
  const position = (await page.evaluate(() => tangentDiagnostics())).position;
  await page.waitForTimeout(150);
  assert.equal(
    (await page.evaluate(() => tangentDiagnostics())).position,
    position,
  );
  // The playing surface exposes only a corner gear; all settings live in pause.
  assert.deepEqual((await page.evaluate(() => tangentDiagnostics())).buttons.map(b => b.id), ["help"]);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing);
  await click(page, "help");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  const pausedAt = (await page.evaluate(() => tangentDiagnostics())).position;
  await page.waitForTimeout(180);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).position, pausedAt);
  await click(page, "fullscreen");
  await page.waitForFunction(
    () => document.fullscreenElement?.id === "tangent",
  );
  await page.keyboard.press("F11");
  await page.waitForFunction(() => !document.fullscreenElement);
  await page.waitForFunction(() => tangentDiagnostics().help);
  await click(page, "theme");
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !tangentDiagnostics().help);
  await click(page, "help");
  const pickerPromise = page.waitForEvent("filechooser");
  await click(page, "import");
  const picker = await pickerPromise;
  await picker.setFiles({
    name: "invalid.mid",
    mimeType: "audio/midi",
    buffer: Buffer.from("not midi"),
  });
  await page.waitForTimeout(150);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).notes, 130);
  const xmlPickerPromise = page.waitForEvent("filechooser");
  await click(page, "import");
  await (await xmlPickerPromise).setFiles(new URL("../../crates/tangent-musicxml/tests/fixtures/piano.musicxml", import.meta.url).pathname);
  await page.waitForFunction(() => tangentDiagnostics().notes === 5);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing && !tangentDiagnostics().help);
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  await click(page, "sample");
  await page.waitForFunction(() => tangentDiagnostics().notes === 130);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing && tangentDiagnostics().energy > 0.0001);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  await page.keyboard.press("Escape");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForFunction(() => tangentDiagnostics().width === 390);
  const ui = await page.evaluate(() => tangentDiagnostics());
  assert(
    ui.buttons.every(
      (b) =>
        b.x >= 0 && b.y >= 0 && b.x + b.w <= ui.width && b.y + b.h <= ui.height,
    ),
  );
  await click(page, "help");
  await page.waitForFunction(() => tangentDiagnostics().help);
  const menu = await page.evaluate(() => tangentDiagnostics());
  assert.equal(menu.playing, false);
  assert(menu.buttons.length > 10);
  assert(menu.buttons.every(b => b.x >= 0 && b.y >= 0 && b.x + b.w <= menu.width && b.y + b.h <= menu.height));
  await page.keyboard.press("Escape");
  assert.equal(
    await page.evaluate(
      () =>
        document.documentElement.scrollWidth === innerWidth &&
        document.documentElement.scrollHeight === innerHeight,
    ),
    true,
  );
  const noGpu = await browser.newPage();
  await noGpu.addInitScript(() => {
    const original = HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext = function (kind, ...args) {
      return kind === "webgl2" ? null : original.call(this, kind, ...args);
    };
  });
  await noGpu.goto(server.url);
  await noGpu.waitForFunction(() =>
    document
      .getElementById("startup")
      .textContent.includes("WebGL2 is unavailable"),
  );
  assert.equal(await noGpu.locator("#startup").isVisible(), true);
  assert.equal(await noGpu.locator("canvas").count(), 1);
  await noGpu.close();
  assert.deepEqual(errors, []);
  console.log(
    "Bevy GPU browser passed: one canvas, in-GPU controls, staff notes, sound, transport/volume shortcuts, fullscreen, settings, MusicXML measures/endings, repeats and responsive viewport.",
  );
} finally {
  await browser?.close();
  await server.close();
}
