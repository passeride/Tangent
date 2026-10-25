// Genuine SQLite bytes persisted atomically in IndexedDB. No UI or scoring rules.
import { migrate, readSettings, writeSettings, schemaVersion } from "./storage-schema.js";
const filename = "tangent.sqlite";
export async function openStorage() {
  const SQL = await globalThis.initSqlJs({ locateFile: name => new URL(`./vendor/${name}`, import.meta.url).href });
  const files = await new Promise((resolve, reject) => {
    const request = indexedDB.open("tangent-local", 1);
    request.onupgradeneeded = () => request.result.createObjectStore("files");
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
    request.onblocked = () => reject(Error("Local database is blocked by another tab."));
  });
  files.onversionchange = () => files.close();
  const diagnostics = { ready: false, pending: 0, revision: 0, schemaVersion, error: null };
  // SQLite work is synchronous inside get.onsuccess: the IDB transaction remains
  // active, serializes competing tabs, and commits the new complete image atomically.
  function transaction(edit, write = true) {
    return new Promise((resolve, reject) => {
      let sqlite, result, failure;
      const tx = files.transaction("files", write ? "readwrite" : "readonly");
      const store = tx.objectStore("files");
      const request = store.get(filename);
      request.onsuccess = () => {
        try {
          sqlite = request.result ? new SQL.Database(new Uint8Array(request.result)) : new SQL.Database();
          migrate(sqlite);
          result = edit(sqlite);
          if (write) store.put(sqlite.export(), filename);
        } catch (error) {
          failure = error;
          tx.abort();
        }
      };
      tx.oncomplete = () => { sqlite?.close(); resolve(result); };
      tx.onabort = () => { sqlite?.close(); reject(failure ?? tx.error ?? Error("Local save failed")); };
    });
  }
  let settings;
  try { settings = await transaction(readSettings); }
  catch (error) { files.close(); throw error; }
  diagnostics.ready = true;
  let tail = Promise.resolve();
  function saveSettings(settings) {
    diagnostics.pending++;
    const result = tail.then(() => transaction(db => writeSettings(db, settings)));
    tail = result.catch(() => {});
    return result.then(() => { diagnostics.revision++; diagnostics.error = null; })
      .catch(error => { diagnostics.error = String(error.message ?? error); throw error; })
      .finally(() => { diagnostics.pending--; });
  }
  return {
    settings, diagnostics, saveSettings,
    async exportDatabase() { await tail; return transaction(db => db.export(), false); },
    close: () => files.close(),
  };
}
