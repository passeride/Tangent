import { chromium } from "playwright";
import { serve, ready, gpuArgs } from "./browser-helpers.mjs";
const server = await serve(new URL("./", import.meta.url).pathname);
let browser;
try {
  browser = await chromium.launch({ headless: true, args: gpuArgs });
  const page = await browser.newPage({
    viewport: { width: 1440, height: 960 },
    deviceScaleFactor: 1,
  });
  await ready(page, server.url);
  await page.keyboard.press("ArrowRight");
  for (const mode of ["blocks", "staff", "notes", "score"]) {
    await page.waitForTimeout(150);
    await page.screenshot({
      path: new URL(`../../docs/screenshots/piano-${mode}.png`, import.meta.url)
        .pathname,
    });
    await page.keyboard.press("KeyV");
  }
  console.log("Captured four real Bevy/wgpu screens.");
} finally {
  await browser?.close();
  await server.close();
}
