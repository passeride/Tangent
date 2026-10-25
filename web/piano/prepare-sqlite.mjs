// Copy exact npm-locked runtime files; no CDN requests at application runtime.
import { mkdir, copyFile } from "node:fs/promises";
const root = new URL("./", import.meta.url);
await mkdir(new URL("vendor/", root), { recursive: true });
for (const name of ["sql-wasm.js", "sql-wasm.wasm"]) {
  await copyFile(new URL(`node_modules/sql.js/dist/${name}`, root), new URL(`vendor/${name}`, root));
}
await copyFile(new URL("node_modules/sql.js/LICENSE", root), new URL("vendor/sql.js-LICENSE.txt", root));
