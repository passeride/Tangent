// Exercise distribution URLs under /Tangent/, matching project GitHub Pages.
import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { resolve, sep } from "node:path";
import { chromium } from "playwright";
const root = resolve(process.argv[2] ?? "dist/piano");
const server = createServer(async (req, res) => {
  try {
    const path = new URL(req.url, "http://localhost").pathname;
    if (!path.startsWith("/Tangent/")) throw Error();
    const file = resolve(
      root,
      decodeURIComponent(path.slice(9)) || "index.html",
    );
    if (!file.startsWith(root + sep)) throw Error();
    res.setHeader(
      "Content-Type",
      {
        wasm: "application/wasm",
        js: "text/javascript",
        css: "text/css",
        html: "text/html",
      }[file.split(".").pop()] ?? "application/octet-stream",
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
  const page = await browser.newPage(),
    errors = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto(`http://127.0.0.1:${server.address().port}/Tangent/`);
  await page.waitForFunction(() => !!window.tangentDiagnostics);
  await page.click("#sample");
  await page.waitForFunction(() => tangentDiagnostics().notes === 228);
  assert.match(await page.locator("#title").innerText(), /Bach/);
  await page.click("#play");
  await page.waitForFunction(() => tangentDiagnostics().position > 300_000);
  await page.waitForFunction(() => tangentDiagnostics().energy > 0.0001);
  assert.deepEqual(errors, []);
  console.log(
    "Packaged Pages site passed: project subpath, real WASM, CC MIDI and sound.",
  );
} finally {
  await browser?.close();
  await new Promise((r) => server.close(r));
}
