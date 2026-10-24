import { notehead } from "./falling.js";
// Time-proportional reading aid. Written durations/measures must come from a
// notation adapter; MIDI glyph duration categories are explicitly approximate.
export function score(ctx, g, frame, theme, reading) {
  const left = 65,
    right = g.width - 40,
    span = 8_000_000,
    page = Math.floor(frame.position_us / span),
    start = page * span;
  const available = g.strike - 65,
    gap = Math.min(12, available / 23),
    treble = 70 + gap * 9,
    bass = treble + gap * 11;
  const pitchY = (p) =>
    (p.bass ? bass : treble) - ((p.step - (p.bass ? 18 : 30)) * gap) / 2;
  ctx.fillStyle = theme.page;
  ctx.fillRect(0, 38, g.width, g.strike - 38);
  ctx.strokeStyle = theme.staff;
  ctx.lineWidth = 1;
  for (const bottom of [treble, bass])
    for (let i = 0; i < 5; i++) {
      ctx.beginPath();
      ctx.moveTo(26, bottom - i * gap);
      ctx.lineTo(right, bottom - i * gap);
      ctx.stroke();
    }
  clef(ctx, 39, treble - gap * 2, gap, "treble", theme.ink);
  clef(ctx, 39, bass - gap * 2, gap, "bass", theme.ink);
  for (let i = 0; i < 4; i++) {
    const x = left + ((right - left) * i) / 4;
    ctx.strokeStyle = theme.staff;
    ctx.beginPath();
    ctx.moveTo(x, treble - gap * 4);
    ctx.lineTo(x, bass);
    ctx.stroke();
  }
  for (const n of frame.notes) {
    if (n.start_us < start || n.start_us >= start + span) continue;
    const p = reading[n.key],
      x = left + ((right - left) * (n.start_us - start)) / span,
      y = pitchY(p);
    const color = theme.colors[n.part % theme.colors.length];
    for (const step of p.ledgers) {
      const ly = pitchY({ ...p, step });
      ctx.strokeStyle = color;
      ctx.beginPath();
      ctx.moveTo(x - 11, ly);
      ctx.lineTo(x + 11, ly);
      ctx.stroke();
    }
    if (p.sharp) {
      ctx.font = "17px serif";
      ctx.fillStyle = color;
      ctx.fillText("♯", x - 21, y + 5);
    }
    const duration = (n.end_us - n.start_us) / 1e6;
    notehead(ctx, x, y, color, {
      hollow: duration >= 0.8,
      stem: duration < 1.6,
      flag: duration < 0.3,
      size: 6,
    });
    // Duration trace preserves performance length without claiming a note value.
    ctx.globalAlpha = 0.22;
    ctx.strokeStyle = color;
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.moveTo(x, y + 11);
    ctx.lineTo(
      Math.min(right, x + ((right - left) * (n.end_us - n.start_us)) / span),
      y + 11,
    );
    ctx.stroke();
    ctx.globalAlpha = 1;
  }
  const x = left + ((right - left) * (frame.position_us - start)) / span;
  ctx.strokeStyle = theme.glow;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(x, treble - gap * 5);
  ctx.lineTo(x, bass + gap * 2);
  ctx.stroke();
}

// Original vector clefs avoid missing musical-symbol glyphs on Linux/web.
function clef(ctx, x, y, gap, type, color) {
  ctx.save();
  ctx.translate(x, y);
  ctx.scale(gap / 10, gap / 10);
  ctx.strokeStyle = color;
  ctx.fillStyle = color;
  ctx.lineWidth = 2;
  ctx.lineCap = "round";
  if (type === "bass") {
    ctx.beginPath();
    ctx.moveTo(-7, -7);
    ctx.bezierCurveTo(-8, -22, 13, -20, 9, -5);
    ctx.bezierCurveTo(7, 4, -1, 9, -8, 13);
    ctx.stroke();
    for (const [dx, dy, r] of [
      [-6, -10, 3],
      [16, -15, 1.8],
      [16, -5, 1.8],
    ]) {
      ctx.beginPath();
      ctx.arc(dx, dy, r, 0, Math.PI * 2);
      ctx.fill();
    }
  } else {
    ctx.beginPath();
    ctx.moveTo(3, 29);
    ctx.bezierCurveTo(15, 38, 10, 42, 4, 41);
    ctx.bezierCurveTo(-2, 40, 0, 34, 3, 34);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(7, 36);
    ctx.lineTo(-1, -25);
    ctx.bezierCurveTo(-7, -40, 9, -43, 7, -26);
    ctx.bezierCurveTo(5, -13, -13, -8, -12, 6);
    ctx.bezierCurveTo(-11, 20, 12, 21, 13, 8);
    ctx.bezierCurveTo(13, -4, -5, -4, -5, 6);
    ctx.bezierCurveTo(-5, 12, 3, 13, 5, 10);
    ctx.stroke();
  }
  ctx.restore();
}
