export function geometry(width, height, layout) {
  const margin = 24,
    keyHeight = Math.min(122, Math.max(82, height * 0.23));
  return {
    width,
    height,
    margin,
    keyHeight,
    strike: height - keyHeight,
    unit: (width - margin * 2) / layout.white_count,
    layout,
  };
}
export function keyRect(g, key) {
  const k = g.layout.keys.find((k) => k.key === key);
  if (!k) return null;
  return {
    x: g.margin + k.x * g.unit,
    w: k.width * g.unit,
    y: g.strike,
    h: g.keyHeight * (k.black ? 0.62 : 1),
    black: k.black,
  };
}
export function center(g, key) {
  const r = keyRect(g, key);
  return r ? r.x + r.w / 2 : null;
}
export function rounded(ctx, x, y, w, h, r = 4) {
  if (h <= 0 || w <= 0) return;
  ctx.beginPath();
  ctx.roundRect(x, y, w, h, Math.min(r, w / 2, h / 2));
  ctx.fill();
}
export function pitchName(key) {
  return (
    ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"][
      key % 12
    ] +
    (Math.floor(key / 12) - 1)
  );
}
