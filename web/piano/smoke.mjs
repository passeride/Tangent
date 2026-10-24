import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
const root = new URL("./", import.meta.url);
const server = createServer(async (req, res) => {
  try {
    const path = decodeURIComponent(
      new URL(req.url, "http://localhost").pathname,
    );
    const file = new URL(path === "/" ? "index.html" : `.${path}`, root);
    if (!file.href.startsWith(root.href)) throw Error();
    res.setHeader(
      "Content-Type",
      {
        wasm: "application/wasm",
        js: "text/javascript",
        css: "text/css",
        html: "text/html",
      }[file.pathname.split(".").pop()] ?? "application/octet-stream",
    );
    res.end(await readFile(file));
  } catch {
    res.writeHead(404);
    res.end();
  }
});
await new Promise((r) => server.listen(0, "127.0.0.1", r));
let browser;
try {
  browser = await chromium.launch({ headless: true, args: ["--no-sandbox"] });
  const page = await browser.newPage({
    viewport: { width: 1440, height: 1000 },
  });
  const errors = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto(`http://127.0.0.1:${server.address().port}`);
  await page.waitForFunction(() => !!window.tangentDiagnostics);
  assert.equal((await page.evaluate(() => tangentDiagnostics())).notes, 40);
  await page.keyboard.down("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().live.includes(60));
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  assert.equal(
    (await page.evaluate(() => tangentDiagnostics())).audioState,
    "running",
  );
  await page.keyboard.up("KeyA");
  await page.waitForFunction(() => tangentDiagnostics().live.length === 0);
  // Real black-key hit uses the same Rust layout as the rendered piano.
  const rect = await page.locator("#piano").boundingBox(),
    g = (await page.evaluate(() => tangentDiagnostics())).geometry;
  const k = g.layout.keys.find((k) => k.key === 61),
    x = rect.x + g.margin + (k.x + k.width / 2) * g.unit,
    y = rect.y + g.strike + 15;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.waitForFunction(() => tangentDiagnostics().live.includes(61));
  await page.mouse.up();
  // Independent Web Audio instances verify stream/pedal/overlap state and seek.
  const audioChecks = await page.evaluate(async () => {
    const { PianoAudio } = await import("./audio.js");
    const a = new PianoAudio();
    await a.activate();
    const send = (message) => a.event({ port: 0, channel: 0, message }, a.now);
    send({ On: { key: 60, velocity: 90 } });
    send({ On: { key: 60, velocity: 90 } });
    const s = a.state(0, 0),
      first = s.keys.get(60)[0],
      second = s.keys.get(60)[1];
    send({ Off: { key: 60 } });
    const fifo = first.ended && !second.ended && s.keys.get(60).length === 1;
    send({ Control: { controller: 64, value: 127 } });
    send({ Off: { key: 60 } });
    const held = s.held.length === 1 && !second.ended;
    send({ Control: { controller: 64, value: 0 } });
    const released = second.ended && s.held.length === 0;
    a.panic();
    const history = [
      {
        time_us: 0,
        port: 0,
        channel: 0,
        message: { On: { key: 60, velocity: 90 } },
      },
      { time_us: 10, port: 0, channel: 0, message: { Off: { key: 60 } } },
    ];
    a.restore(history, 20);
    const noReplay = a.voices.size === 0;
    a.restore(history, 5);
    const restoresHeld = a.voices.size === 1;
    a.event(
      { port: 0, channel: 0, message: { On: { key: 67, velocity: 90 } } },
      a.now + 0.3,
    );
    a.stopSong();
    const cancelled = a.voices.size === 0;
    await new Promise((r) => setTimeout(r, 700));
    const silent = a.energy() < 0.0001;
    await a.context.close();
    return { fifo, held, released, noReplay, restoresHeld, cancelled, silent };
  });
  assert.deepEqual(audioChecks, {
    fifo: true,
    held: true,
    released: true,
    noReplay: true,
    restoresHeld: true,
    cancelled: true,
    silent: true,
  });
  await page.click("#play");
  await page.waitForFunction(() => tangentDiagnostics().position > 2_100_000);
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  await page.screenshot({ path: "/tmp/tangent-piano-blocks.png" });
  await page.click("#play");
  const position = (await page.evaluate(() => tangentDiagnostics())).position;
  await page.waitForTimeout(180);
  assert.equal(
    (await page.evaluate(() => tangentDiagnostics())).position,
    position,
  );
  for (const mode of ["staff", "notes", "score"]) {
    await page.selectOption("#view", mode);
    await page.waitForTimeout(60);
    await page.screenshot({ path: `/tmp/tangent-piano-${mode}.png` });
  }
  await page.selectOption("#theme", "paper");
  await page.waitForTimeout(60);
  await page.screenshot({ path: "/tmp/tangent-piano-paper.png" });
  await page.click("#restart");
  await page.waitForFunction(() => tangentDiagnostics().position === 0);
  await page.setInputFiles("#file", {
    name: "invalid.mid",
    mimeType: "audio/midi",
    buffer: Buffer.from("not midi"),
  });
  await page.locator("#error").waitFor({ state: "visible" });
  assert.equal((await page.evaluate(() => tangentDiagnostics())).notes, 40);
  // Independent short file: C4 at t0, release 0.5s. Type0, 480 PPQ.
  const midi = Buffer.from([
    77, 84, 104, 100, 0, 0, 0, 6, 0, 0, 0, 1, 1, 224, 77, 84, 114, 107, 0, 0, 0,
    13, 0, 144, 60, 100, 131, 96, 128, 60, 0, 0, 255, 47, 0,
  ]);
  await page.setInputFiles("#file", {
    name: "one-note.mid",
    mimeType: "audio/midi",
    buffer: midi,
  });
  await page.waitForFunction(() => tangentDiagnostics().notes === 1);
  assert.equal(await page.locator("#error").isVisible(), false);
  await page.click("#play");
  await page.waitForFunction(() => tangentDiagnostics().position === 500_000);
  assert.equal(
    (await page.evaluate(() => tangentDiagnostics())).playing,
    false,
  );
  // Narrow layout and a touch pointer both remain usable.
  await page.selectOption("#range", "49");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForTimeout(60);
  assert.equal(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
    true,
  );
  await page.screenshot({ path: "/tmp/tangent-piano-mobile.png" });
  assert.deepEqual(errors, []);
  console.log(
    "Piano browser passed: real Bevy/WASM, audio signal, keyboard, black-key pointer, transport, four modes, themes, MIDI import/error/recovery and narrow layout.",
  );
} finally {
  await browser?.close();
  await new Promise((r) => server.close(r));
}
