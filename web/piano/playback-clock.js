// Source timestamps remain musical microseconds. Only their wall-clock mapping changes.
export class PlaybackClock {
  reset(now, position, percent = 100) {
    this.speed = Math.min(200, Math.max(25, percent)) / 100;
    this.anchor = now - position / 1e6 / this.speed;
  }
  position(now, lookahead = 0) { return (now - this.anchor + lookahead) * this.speed * 1e6; }
  time(songUs) { return this.anchor + songUs / 1e6 / this.speed; }
}
