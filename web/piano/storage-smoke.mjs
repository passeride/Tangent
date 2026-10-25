import assert from "node:assert/strict";
import { readFile, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import initSqlJs from "sql.js";
import { chromium } from "playwright";
import { serve, ready, click, gpuArgs } from "./browser-helpers.mjs";
const canonical = p => ({ ...p, volume: Math.fround(p.volume) });
const server = await serve(new URL("./", import.meta.url).pathname);
const profile = await mkdtemp(join(tmpdir(), "tangent-storage-"));
let context;
try {
  context = await chromium.launchPersistentContext(profile, { headless: true, args: gpuArgs });
  let page = await context.newPage();
  await ready(page, server.url);
  await page.waitForFunction(() => tangentDiagnostics().storage.ready);
  await click(page, "settings");
  for (const [tab, id] of [
    ["settings-visuals", "theme"],
    ["settings-audio", "instrument"],
    ["settings-effects", "effects"],
    ["settings-visuals", "view"],
    ["settings-visuals", "range"],
    ["settings-audio", "quieter"],
    ["settings-audio", "hear"],
  ]) {
    await click(page, tab);
    const before = await page.evaluate(() => JSON.stringify(tangentDiagnostics().preferences));
    await click(page, id);
    await page.waitForFunction(before => JSON.stringify(tangentDiagnostics().preferences) !== before, before);
  }
  let expected = await page.evaluate(() => tangentDiagnostics().preferences);
  await page.waitForFunction(() => tangentDiagnostics().storage.pending === 0 && tangentDiagnostics().storage.revision >= 7);
  await click(page, "settings-accessibility");
  await click(page, "reduced-motion");
  await click(page, "settings-effects");
  await click(page, "effect-strength");
  await page.waitForFunction(()=>tangentDiagnostics().preferences.reduced_motion && tangentDiagnostics().preferences.effect_strength===2);
  expected = await page.evaluate(()=>tangentDiagnostics().preferences);
  await page.waitForFunction(()=>tangentDiagnostics().storage.pending===0);
  // A full browser restart uses the same durable profile and origin.
  await context.close();
  context = await chromium.launchPersistentContext(profile, { headless: true, args: gpuArgs });
  page = await context.newPage();
  await ready(page, server.url);
  assert.deepEqual(canonical(await page.evaluate(() => tangentDiagnostics().preferences)), canonical(expected));
  assert.equal(await page.evaluate(() => tangentDiagnostics().playing), false);
  assert.equal(await page.evaluate(() => tangentDiagnostics().screen), "Main");
  await click(page, "settings");
  await click(page, "settings-data");
  const downloadPromise = page.waitForEvent("download");
  await click(page, "export-database");
  const download = await downloadPromise;
  const SQL = await initSqlJs();
  const db = new SQL.Database(await readFile(await download.path()));
  assert.equal(db.exec("PRAGMA integrity_check")[0].values[0][0], "ok");
  assert.deepEqual(canonical(JSON.parse(db.exec("SELECT payload FROM settings")[0].values[0][0])), canonical(expected));
  assert.equal(db.exec("SELECT count(*) FROM scores")[0].values[0][0], 0);
  assert.equal(db.exec("SELECT count(*) FROM practice_sessions")[0].values[0][0], 0);
  db.close();
  await page.evaluate(() => {
    window.originalPut = IDBObjectStore.prototype.put;
    IDBObjectStore.prototype.put = function() { throw new DOMException("Full for test", "QuotaExceededError"); };
  });
  await click(page, "settings-visuals");
  await click(page, "theme");
  await page.waitForFunction(() => tangentDiagnostics().storage.error !== null);
  await page.waitForFunction(() => tangentDiagnostics().storage_status.includes("Could not save"));
  await page.evaluate(() => { IDBObjectStore.prototype.put = window.originalPut; });
  assert.deepEqual(canonical(await page.evaluate(async () => { const { openStorage } = await import("./storage.js"); const db = await openStorage(); const value = db.settings; db.close(); return value; })), canonical(expected));
  // Concurrent connections must merge against the latest SQLite image, not a stale snapshot.
  await page.evaluate(async () => {
    const { openStorage } = await import("./storage.js");
    const a = await openStorage(), b = await openStorage();
    await Promise.all([a.saveSettings({ version: 1, marker: "first" }), b.saveSettings({ version: 1, marker: "last" })]);
    const bytes = await b.exportDatabase();
    const SQL = await window.initSqlJs({ locateFile: n => new URL(`./vendor/${n}`, location.href).href });
    const db = new SQL.Database(bytes);
    if (JSON.parse(db.exec("SELECT payload FROM settings")[0].values[0][0]).marker !== "last") throw Error("Lost concurrent save");
    if (db.exec("PRAGMA integrity_check")[0].values[0][0] !== "ok") throw Error("Invalid snapshot");
    db.close(); a.close(); b.close();
  });
  const failed = await context.newPage();
  await failed.addInitScript(() => {
    Object.defineProperty(window, "indexedDB", { get() { throw Error("Storage disabled for test"); } });
  });
  await ready(failed, server.url);
  assert.equal(await failed.evaluate(() => tangentDiagnostics().storage.ready), false);
  assert.match(await failed.evaluate(() => tangentDiagnostics().storage_status), /unavailable/);
  await click(failed, "freeform");
  await failed.waitForFunction(() => tangentDiagnostics().screen === "Session");
  await failed.keyboard.down("KeyA");
  await failed.waitForFunction(() => tangentDiagnostics().live.includes(60));
  await failed.keyboard.up("KeyA");
  console.log("Storage browser passed: settings survive browser restart, valid SQLite backup, no invented scores, concurrent saves and playable storage-failure fallback.");
} finally {
  await context?.close();
  await server.close();
  await rm(profile, { recursive: true, force: true });
}
