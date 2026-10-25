import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { deflateRawSync } from "node:zlib";

// In-memory MXL fixture built by the browser smoke test, not a downloaded score.
function crc32(data) {
  let crc = 0xffffffff;
  for (const byte of data) {
    crc ^= byte;
    for (let i = 0; i < 8; i++)
      crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
  }
  return (crc ^ 0xffffffff) >>> 0;
}
function compressedMxl(score) {
  const manifest = Buffer.from(
    '<container><rootfiles><rootfile full-path="score.musicxml"/></rootfiles></container>',
  );
  const files = [
    ["META-INF/container.xml", manifest, false],
    ["score.musicxml", score, true],
  ];
  const locals = [], central = [];
  let offset = 0;
  for (const [filename, data, deflate] of files) {
    const name = Buffer.from(filename);
    const packed = deflate ? deflateRawSync(data) : data;
    const method = deflate ? 8 : 0;
    const crc = crc32(data);
    const local = Buffer.alloc(30);
    local.writeUInt32LE(0x04034b50, 0);
    local.writeUInt16LE(20, 4);
    local.writeUInt16LE(method, 8);
    local.writeUInt32LE(crc, 14);
    local.writeUInt32LE(packed.length, 18);
    local.writeUInt32LE(data.length, 22);
    local.writeUInt16LE(name.length, 26);
    locals.push(local, name, packed);
    const item = Buffer.alloc(46);
    item.writeUInt32LE(0x02014b50, 0);
    item.writeUInt16LE(20, 4);
    item.writeUInt16LE(20, 6);
    item.writeUInt16LE(method, 10);
    item.writeUInt32LE(crc, 16);
    item.writeUInt32LE(packed.length, 20);
    item.writeUInt32LE(data.length, 24);
    item.writeUInt16LE(name.length, 28);
    item.writeUInt32LE(offset, 42);
    central.push(item, name);
    offset += local.length + name.length + packed.length;
  }
  const directory = Buffer.concat(central);
  const end = Buffer.alloc(22);
  end.writeUInt32LE(0x06054b50, 0);
  end.writeUInt16LE(files.length, 8);
  end.writeUInt16LE(files.length, 10);
  end.writeUInt32LE(directory.length, 12);
  end.writeUInt32LE(offset, 16);
  return Buffer.concat([...locals, directory, end]);
}

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
  assert.equal((await page.evaluate(() => tangentDiagnostics())).screen, "Main");
  await page.keyboard.press("Tab");
  await page.waitForFunction(()=>tangentDiagnostics().focused_control==='start-play');
  await page.keyboard.press("Enter");
  await page.waitForFunction(()=>tangentDiagnostics().screen==='Session');
  await page.keyboard.press("Escape"); await click(page, "main");
  await page.keyboard.press("KeyA");
  assert.equal((await page.evaluate(() => tangentDiagnostics())).live.length, 0);
  await click(page, "settings");
  assert.equal((await page.evaluate(() => tangentDiagnostics())).settings_tab, "Audio & MIDI");
  await page.keyboard.press("Escape");
  await click(page, "train");
  await click(page, "train-notes");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
  assert.equal((await page.evaluate(() => tangentDiagnostics())).vfx_scale, 1.125);
  await click(page, "practice-listen");
  await page.waitForFunction(()=>tangentDiagnostics().energy > 0.0001);
  assert.equal((await page.evaluate(()=>tangentDiagnostics())).completed, 0);
  await page.waitForFunction(()=>tangentDiagnostics().reference_keys.length===0);
  await page.keyboard.down("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().practice_waiting_release === true);
  await page.waitForFunction(() => tangentDiagnostics().vfx_particles >= 20
    && tangentDiagnostics().vfx_ripples > 0);
  await page.keyboard.up("KeyA");
  await page.keyboard.press("Escape");
  await click(page, "main");
  await click(page, "train");
  await click(page, "train-chords");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
  await page.keyboard.down("KeyA");
  await page.keyboard.down("KeyD");
  await page.keyboard.down("KeyG");
  await page.waitForFunction(() => tangentDiagnostics().practice_waiting_release === true);
  await page.keyboard.up("KeyA");
  await page.keyboard.up("KeyD");
  await page.keyboard.up("KeyG");
  await page.keyboard.press("Escape");
  await click(page, "main");
  await click(page, "freeform");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
  await page.keyboard.press("Space");
  assert.equal((await page.evaluate(() => tangentDiagnostics())).playing, false);
  await page.keyboard.press("Escape");
  await click(page, "main");
  await click(page, "start-play");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
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
  // Common transport remains reachable; Settings is a separate page.
  assert.deepEqual((await page.evaluate(() => tangentDiagnostics())).buttons.map(b => b.id), ["play", "help"]);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing);
  await click(page, "help");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  const pausedAt = (await page.evaluate(() => tangentDiagnostics())).position;
  await page.waitForTimeout(180);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).position, pausedAt);
  await click(page, "settings");
  await page.waitForFunction(() => tangentDiagnostics().settings_open);
  await click(page, "settings-visuals");
  await click(page, "fullscreen");
  await page.waitForFunction(
    () => document.fullscreenElement?.id === "tangent",
  );
  await page.keyboard.press("F11");
  await page.waitForFunction(() => !document.fullscreenElement);
  await page.waitForFunction(() => tangentDiagnostics().settings_open);
  await click(page, "theme");
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().settings_open);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !tangentDiagnostics().help);
  await click(page, "help");
  await click(page, "settings");
  await click(page, "settings-data");
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
  await click(page, "sample");
  await page.waitForFunction(() => tangentDiagnostics().notes === 130);
  const mxlPickerPromise = page.waitForEvent("filechooser");
  await click(page, "import");
  const mxl = compressedMxl(readFileSync(
    new URL("../../crates/tangent-musicxml/tests/fixtures/piano.musicxml", import.meta.url),
  ));
  await (await mxlPickerPromise).setFiles({
    name: "piano.mxl",
    mimeType: "application/vnd.recordare.musicxml",
    buffer: mxl,
  });
  await page.waitForFunction(() => tangentDiagnostics().notes === 5);
  await click(page, "settings-back");
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing && !tangentDiagnostics().help);
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  await click(page, "settings");
  await click(page, "settings-data");
  await click(page, "sample");
  await page.waitForFunction(() => tangentDiagnostics().notes === 130);
  await click(page, "settings-back");
  await page.keyboard.press("Space");
  await page.waitForFunction(() => tangentDiagnostics().playing && tangentDiagnostics().energy > 0.0001);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => tangentDiagnostics().help && !tangentDiagnostics().playing);
  await page.keyboard.press("Escape");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForFunction(() => tangentDiagnostics().width === 390);
  const ui = await page.evaluate(() => tangentDiagnostics());
  assert.ok(Math.abs(ui.vfx_scale - 0.65) < 1e-6, "Mobile viewport has a bounded effect scale");
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
  assert(menu.buttons.length >= 8);
  assert(menu.buttons.every(b => b.x >= 0 && b.y >= 0 && b.x + b.w <= menu.width && b.y + b.h <= menu.height));
  await click(page, "settings");
  const settings = await page.evaluate(() => tangentDiagnostics());
  assert.equal(settings.settings_open, true);
  assert(settings.buttons.every(b => b.x >= 0 && b.y >= 0 && b.x + b.w <= settings.width && b.y + b.h <= settings.height));
  await click(page, "settings-back");
  await page.keyboard.press("Escape");
  assert.equal(
    await page.evaluate(
      () =>
        document.documentElement.scrollWidth === innerWidth &&
        document.documentElement.scrollHeight === innerHeight,
    ),
    true,
  );
  await click(page, "help");
  await click(page, "main");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Main");
  for (const id of ["train", "train-notes"]) {
    const screen = await page.evaluate(() => tangentDiagnostics());
    assert(screen.buttons.every(b => b.x >= 0 && b.y >= 0 && b.x + b.w <= screen.width && b.y + b.h <= screen.height));
    await click(page, id);
  }
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
  await page.keyboard.down("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().practice_waiting_release === true);
  await page.keyboard.up("KeyA");
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
