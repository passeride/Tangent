export class Particles {
  items = [];
  emit(x, y, color) {
    for (let i = 0; i < 12; i++)
      this.items.push({
        x,
        y,
        vx: (Math.random() - 0.5) * 85,
        vy: -30 - Math.random() * 100,
        life: 0,
        max: 0.35 + Math.random() * 0.65,
        color,
        size: 1 + Math.random() * 2,
      });
    if (this.items.length > 500) this.items.splice(0, this.items.length - 500);
  }
  draw(ctx, dt, enabled) {
    if (!enabled) {
      this.items = [];
      return;
    }
    this.items = this.items.filter((p) => p.life < p.max);
    for (const p of this.items) {
      p.life += dt;
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.vy += 65 * dt;
      ctx.globalAlpha = Math.max(0, 1 - p.life / p.max);
      ctx.fillStyle = p.color;
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  }
}
