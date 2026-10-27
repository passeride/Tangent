if(process.env.TANGENT_CAPTURE_METADATA==='1'){await import('./metadata-smoke.mjs');process.exit(0);}
if (process.env.TANGENT_CAPTURE_BEAMS === '1') {
  await import('./beaming-smoke.mjs');
  process.exit(0);
}
if (process.env.TANGENT_CAPTURE_GUIDED === '1') {
  await import('./guided-smoke.mjs');
  process.exit(0);
}
if (process.env.TANGENT_CAPTURE_FLOW === '1') {
  await import('./flow-smoke.mjs');
  process.exit(0);
}
if (process.env.TANGENT_CAPTURE_EXPLORER === '1') {
  await import('./explorer-smoke.mjs');
  process.exit(0);
}
// Optional dedicated real-app 3D piano captures.
if (process.env.TANGENT_CAPTURE_3D === '1') {
  await import('./piano3d-smoke.mjs');
  process.exit(0);
}
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
// Product-strategy captures are made by real input scenarios in the same app.
if (process.env.TANGENT_CAPTURE_PRODUCT === '1') {
  const {spawn}=await import('node:child_process');
  for (const file of ['passage-smoke.mjs','hud-smoke.mjs','goal-smoke.mjs']) {
    await new Promise((resolve,reject)=>{
      const child=spawn(process.execPath,[new URL(file,import.meta.url).pathname],{cwd:new URL('./',import.meta.url),stdio:'inherit'});
      child.on('error',reject);
      child.on('exit',code=>code===0?resolve():reject(new Error(`${file} failed: ${code}`)));
    });
  }
  process.exit(0);
}
const server = await serve(new URL("./", import.meta.url).pathname);
let browser;
try {
  browser = await chromium.launch({ headless: true, args: gpuArgs });
  const page = await browser.newPage({
    viewport: { width: 1440, height: 960 },
    deviceScaleFactor: 1,
  });
  await ready(page, server.url);
  await page.screenshot({ path: new URL("../../docs/screenshots/main-menu.png", import.meta.url).pathname });
  await click(page, "train");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Train");
  await page.waitForTimeout(250);
  await page.screenshot({ path: new URL("../../docs/screenshots/train-menu.png", import.meta.url).pathname });
  await click(page, "train-notes");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
  await page.waitForTimeout(250);
  await page.screenshot({ path: new URL("../../docs/screenshots/train-notes.png", import.meta.url).pathname });
  await page.keyboard.press("Escape");
  await click(page, "main");
  await click(page, "start-play");
  await page.waitForFunction(() => tangentDiagnostics().screen === "Session");
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
  console.log("Captured real Bevy/wgpu menus, practice and player screens, including written first/second endings.");
} finally {
  await browser?.close();
  await server.close();
}

// Optional reproducible scored-session/history screenshots from real Bevy input.
if (process.env.TANGENT_CAPTURE_PROGRESS === '1') {
  await import('./progress-smoke.mjs');
  const {copyFile}=await import('node:fs/promises');
  for(const name of ['practice-results','practice-progress']) {
    await copyFile(new URL(`../../dist/verification/${name}.png`,import.meta.url),new URL(`../../docs/screenshots/${name}.png`,import.meta.url));
  }
}
