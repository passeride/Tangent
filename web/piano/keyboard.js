import { keyRect, rounded, pitchName } from "./geometry.js";
export function drawKeyboard(ctx, g, theme, active) {
  for (const black of [false, true])
    for (const k of g.layout.keys.filter((k) => k.black === black)) {
      const r = keyRect(g, k.key),
        color = active.get(k.key);
      const gradient = ctx.createLinearGradient(0, r.y, 0, r.y + r.h);
      gradient.addColorStop(0, color ?? (black ? "#26343c" : theme.white));
      gradient.addColorStop(1, color ?? (black ? theme.black : "#bac7c3"));
      ctx.fillStyle = gradient;
      rounded(ctx, r.x + 1, r.y + 1, r.w - 2, r.h - 3, black ? 3 : 4);
      if (color) {
        ctx.fillStyle = color;
        ctx.globalAlpha = 0.27;
        rounded(ctx, r.x, r.y - 11, r.w, 17, 5);
        ctx.globalAlpha = 1;
      }
      if (!black && k.key % 12 === 0) {
        ctx.fillStyle = "#425852";
        ctx.font = `${Math.max(8, Math.min(11, g.unit * 0.45))}px system-ui`;
        ctx.textAlign = "center";
        ctx.fillText(pitchName(k.key), r.x + r.w / 2, r.y + r.h - 12);
      }
    }
  ctx.strokeStyle = theme.glow;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(g.margin, g.strike);
  ctx.lineTo(g.width - g.margin, g.strike);
  ctx.stroke();
}
