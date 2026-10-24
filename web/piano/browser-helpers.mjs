import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { resolve, sep } from "node:path";
export const gpuArgs = [
  "--no-sandbox",
  "--use-gl=angle",
  "--use-angle=swiftshader",
  "--enable-unsafe-swiftshader",
];
export async function serve(root, prefix = "/") {
  root = resolve(root);
  const server = createServer(async (req, res) => {
    try {
      const path = decodeURIComponent(
        new URL(req.url, "http://localhost").pathname,
      );
      if (!path.startsWith(prefix)) throw Error();
      const file = resolve(root, path.slice(prefix.length) || "index.html");
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
  return {
    url: `http://127.0.0.1:${server.address().port}${prefix}`,
    close: () => new Promise((r) => server.close(r)),
  };
}
export async function ready(page, url) {
  page.setDefaultTimeout(90_000);
  await page.goto(url);
  await page.waitForFunction(
    () => window.tangentDiagnostics?.()?.renderer === "bevy-wgpu",
    null,
    { timeout: 90_000 },
  );
  await page.waitForTimeout(500);
}
export async function click(page, id) {
  const b = await page.evaluate(
    (id) => tangentDiagnostics().buttons.find((b) => b.id === id),
    id,
  );
  if (!b) throw Error("Missing Bevy button: " + id);
  await page.mouse.click(b.x + b.w / 2, b.y + b.h / 2);
}
