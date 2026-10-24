import { center, keyRect, rounded } from "./geometry.js";
// Drawn notehead/stem/flag: original vector geometry, no external music font.
export function notehead(
  ctx,
  x,
  y,
  color,
  { hollow = false, flag = false, stem = true, size = 6 } = {},
) {
  ctx.save();
  ctx.strokeStyle = color;
  ctx.fillStyle = color;
  ctx.lineWidth = 1.8;
  ctx.beginPath();
  ctx.ellipse(x, y, size * 1.15, size * 0.75, -0.35, 0, Math.PI * 2);
  hollow ? ctx.stroke() : ctx.fill();
  if (stem) {
    ctx.beginPath();
    ctx.moveTo(x + size, y);
    ctx.lineTo(x + size, y - size * 4.6);
    ctx.stroke();
    if (flag) {
      ctx.beginPath();
      ctx.moveTo(x + size, y - size * 4.6);
      ctx.bezierCurveTo(
        x + size * 1.1,
        y - size * 2.9,
        x + size * 3.2,
        y - size * 3.5,
        x + size * 1.5,
        y - size * 1.4,
      );
      ctx.bezierCurveTo(
        x + size * 2.4,
        y - size * 3,
        x + size * 0.7,
        y - size * 3,
        x + size,
        y - size * 4.6,
      );
      ctx.fill();
    }
  }
  ctx.restore();
}
export function falling(ctx, g, frame, theme, mode, reading) {
  const pps = Math.max(40, (g.strike - 40) / 5.5),
    time = frame.position_us / 1e6;
  ctx.save();
  ctx.beginPath();
  ctx.rect(0, 38, g.width, g.strike - 38);
  ctx.clip();
  // Quiet octave guides in block and glyph modes.
  for (const k of g.layout.keys)
    if (k.key % 12 === 0) {
      const x = center(g, k.key);
      ctx.strokeStyle = theme.staff;
      ctx.globalAlpha = 0.25;
      ctx.beginPath();
      ctx.moveTo(x, 38);
      ctx.lineTo(x, g.strike);
      ctx.stroke();
    }
  ctx.globalAlpha = 1;
  if (mode === "staff") {
    // Rotated bass/treble staff: five long lines per clef, aligned to white keys.
    for (const key of [43, 47, 50, 53, 57, 64, 67, 71, 74, 77]) {
      const x = center(g, key);
      if (x === null) continue;
      ctx.strokeStyle = theme.staff;
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(x, 38);
      ctx.lineTo(x, g.strike);
      ctx.stroke();
    }
    ctx.fillStyle = theme.ink;
    ctx.globalAlpha = 0.45;
    ctx.font = "11px system-ui";
    ctx.textAlign = "center";
    for (const [key, name] of [
      [50, "BASS · F"],
      [71, "TREBLE · G"],
    ])
      ctx.fillText(name, center(g, key), 58);
    ctx.globalAlpha = 1;
  }
  for (const n of frame.notes) {
    const r = keyRect(g, n.key);
    if (!r) continue;
    const onsetY = g.strike - (n.start_us / 1e6 - time) * pps,
      endY = g.strike - (n.end_us / 1e6 - time) * pps,
      color = theme.colors[n.part % theme.colors.length];
    if (n.end_us <= frame.position_us || onsetY < 25) continue;
    if (mode === "notes") {
      // Heads arrive at onset, then remain at the key through their entire span.
      const y = Math.min(g.strike - 8, onsetY),
        x = r.x + r.w / 2,
        duration = (n.end_us - n.start_us) / 1e6;
      if (y < 30) continue;
      if (onsetY > g.strike) {
        const progress =
          (frame.position_us - n.start_us) / Math.max(1, n.end_us - n.start_us);
        ctx.strokeStyle = color;
        ctx.globalAlpha = 0.4;
        ctx.beginPath();
        ctx.arc(
          x,
          y,
          13,
          -Math.PI / 2,
          -Math.PI / 2 + Math.PI * 2 * (1 - progress),
        );
        ctx.stroke();
        ctx.globalAlpha = 1;
      }
      notehead(ctx, x, y, color, {
        hollow: duration >= 0.8,
        flag: duration < 0.3,
        size: Math.min(7, g.unit * 0.5),
      });
    } else {
      const y = Math.max(35, endY),
        bottom = Math.min(g.strike, onsetY),
        w = Math.max(3, r.w * (r.black ? 0.8 : 0.72)),
        x = r.x + (r.w - w) / 2;
      ctx.shadowColor = color;
      ctx.shadowBlur = 7;
      ctx.fillStyle = color;
      ctx.globalAlpha = 0.86;
      rounded(ctx, x, y, w, Math.max(4, bottom - y), 4);
      ctx.shadowBlur = 0;
      ctx.globalAlpha = 1;
      ctx.fillStyle = "#ffffff66";
      rounded(ctx, x + 2, y + 2, Math.max(1, w - 4), 2, 1);
      if (mode === "staff") {
        const rp = reading[n.key];
        for (const step of rp.ledgers) {
          const candidate = reading.find((p) => p.step === step && !p.sharp);
          const lx = candidate ? center(g, candidate.key) : null;
          if (lx !== null) {
            ctx.strokeStyle = theme.ink;
            ctx.lineWidth = 1;
            ctx.beginPath();
            ctx.moveTo(lx, bottom - 10);
            ctx.lineTo(lx, bottom + 5);
            ctx.stroke();
          }
        }
        if (rp.sharp) {
          ctx.fillStyle = theme.ink;
          ctx.font = "12px serif";
          ctx.textAlign = "center";
          ctx.fillText("♯", x - 4, bottom - 3);
        }
      }
    }
  }
  ctx.restore();
}
