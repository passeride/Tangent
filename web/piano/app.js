// Bootstrap only. Every application control and visible element is Bevy UI.
import { installPlatform } from "./platform.js";
import init, { run_player } from "./pkg/tangent_player.js";
installPlatform();
try {
  await init();
  run_player();
} catch (error) {
  document.getElementById("startup").textContent =
    `Tangent could not start: ${error.message ?? error}. WebGL2 is required.`;
}
