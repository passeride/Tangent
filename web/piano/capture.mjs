import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
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
  for (const [mode, view] of [["blocks","Blocks"],["staff","Staff"],["notes","Notes"],["score","Score"]]) {
    if (mode !== "blocks") {
      await page.keyboard.press("KeyV");
      await page.waitForFunction(v => tangentDiagnostics().view === v, view);
    }
    await page.waitForTimeout(700);
    await page.screenshot({ path: new URL(`../../docs/screenshots/piano-${mode}.png`, import.meta.url).pathname });
  }
  await page.keyboard.press("Home");
  await page.waitForFunction(() => tangentDiagnostics().position === 0);
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => tangentDiagnostics().position === 5_000_000);
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => tangentDiagnostics().position === 10_000_000);
  await page.waitForTimeout(700);
  await page.screenshot({ path: new URL("../../docs/screenshots/piano-endings.png", import.meta.url).pathname });
  await click(page, "help");
  await page.waitForTimeout(700);
  await page.screenshot({ path: new URL("../../docs/screenshots/piano-pause.png", import.meta.url).pathname });
  console.log("Captured six real Bevy/wgpu screens, including written first/second endings.");
} finally {
  await browser?.close();
  await server.close();
}
