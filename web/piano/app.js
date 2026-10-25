// Bootstrap only. Every application control and visible element is Bevy UI.
import { installPlatform } from "./platform.js";
import init, { run_player } from "./pkg/tangent_player.js";
const startup = document.getElementById("startup");
function fatal(error) {
  startup.hidden = false;
  startup.textContent = `Tangent could not start: ${error?.message ?? error ?? "Unknown renderer error"}`;
}
// Bevy initializes its GPU asynchronously, so a synchronous try/catch alone
// cannot report all renderer failures. Keep the startup fallback visible.
window.addEventListener("error", (event) =>
  fatal(event.error ?? event.message),
);
window.addEventListener("unhandledrejection", (event) => fatal(event.reason));
try {
  // Probe a detached canvas, never the Bevy-owned canvas: taking its context
  // here would interfere with wgpu's choice of surface attributes.
  const probe = document.createElement("canvas");
  const context = probe.getContext("webgl2");
  if (!context)
    throw new Error(
      "WebGL2 is unavailable. Enable hardware acceleration or use a browser/device with WebGL2 support.",
    );
  context.getExtension("WEBGL_lose_context")?.loseContext();
  await installPlatform();
  await init();
  run_player();
} catch (error) {
  fatal(error);
}
