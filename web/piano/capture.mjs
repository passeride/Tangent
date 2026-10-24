// Reproducible screenshots of the actual app, using its original study.
import { createServer } from "node:http";
import { readFile, mkdir } from "node:fs/promises";
import { chromium } from "playwright";
const root = new URL("./", import.meta.url),
  output = new URL("../../docs/screenshots/", import.meta.url);
await mkdir(output, { recursive: true });
const server = createServer(async (req, res) => {
  try {
    const file = new URL(
      new URL(req.url, "http://localhost").pathname.slice(1) || "index.html",
      root,
    );
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
    viewport: { width: 1440, height: 1080 },
    deviceScaleFactor: 1,
  });
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  await page.waitForFunction(() => !!window.tangentDiagnostics);
  await page.locator("#seek").evaluate((el) => {
    el.value = "2100";
    el.dispatchEvent(new Event("input", { bubbles: true }));
  });
  for (const mode of ["blocks", "staff", "notes", "score"]) {
    await page.selectOption("#view", mode);
    await page.waitForTimeout(80);
    await page.screenshot({
      path: new URL(`piano-${mode}.png`, output).pathname,
      fullPage: true,
    });
  }
  console.log("Captured four actual app views in docs/screenshots/.");
} finally {
  await browser?.close();
  await new Promise((r) => server.close(r));
}
