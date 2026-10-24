import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
const root = new URL("./", import.meta.url);
const server = createServer(async (request, response) => {
  try {
    const path = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
    const file = new URL(path === "/" ? "index.html" : `.${path}`, root);
    if (!file.href.startsWith(root.href)) { response.writeHead(403); response.end(); return; }
    const types = { wasm: "application/wasm", js: "text/javascript", html: "text/html" };
    response.setHeader("Content-Type", types[file.pathname.split(".").pop()] || "application/octet-stream");
    response.end(await readFile(fileURLToPath(file)));
  } catch { response.writeHead(404); response.end(); }
});
await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
let browser;
try {
  browser = await chromium.launch({ headless: true, args: ["--no-sandbox"] });
  const page = await browser.newPage();
  const errors = []; page.on("pageerror", error => errors.push(error.message));
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  await page.waitForFunction(() => !document.getElementById("demo").disabled);
  await page.click("#demo");
  await page.waitForFunction(() => document.getElementById("status").textContent.startsWith("Import complete"));
  const imported = JSON.parse(await page.locator("#json").textContent());
  assert.equal(imported.tracks[0].notes.length, 3);
  assert.equal(imported.duration_us, 1_500_000);
  assert.equal(imported.diagnostics.length, 0);
  await page.setInputFiles("#file", { name: "invalid.mid", mimeType: "audio/midi", buffer: Buffer.from("not midi") });
  await page.locator("#error").waitFor({ state: "visible" });
  assert.match(await page.locator("#error").textContent(), /InvalidFile/);
  await page.click("#demo");
  await page.waitForFunction(() => document.getElementById("status").textContent.startsWith("Import complete"));
  assert.equal(await page.locator("#error").isVisible(), false);
  assert.deepEqual(errors, []);
  console.log("Browser smoke passed: real WASM import, malformed-file error and recovery.");
} finally {
  await browser?.close();
  await new Promise(resolve => server.close(resolve));
}
