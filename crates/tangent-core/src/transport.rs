/// Transport never reads wall time. The platform passes elapsed audio-clock time.
#[derive(Debug, Default)]
pub struct Transport {
    pub position_us: u64,
    pub playing: bool,
    pub duration_us: u64,
}
impl Transport {
    pub fn advance(&mut self, elapsed_us: u64) {
        if self.playing {
            self.position_us = self
                .position_us
                .saturating_add(elapsed_us)
                .min(self.duration_us);
            if self.position_us == self.duration_us {
                self.playing = false;
            }
        }
    }
    pub fn seek(&mut self, position_us: u64) {
        self.position_us = position_us.min(self.duration_us);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pause_seek_end_are_bounded() {
        let mut t = Transport {
            duration_us: 100,
            playing: true,
            ..Default::default()
        };
        t.advance(30);
        t.playing = false;
        t.advance(80);
        assert_eq!(t.position_us, 30);
        t.seek(200);
        assert_eq!(t.position_us, 100);
        t.seek(20);
        t.playing = true;
        t.advance(u64::MAX);
        assert_eq!(t.position_us, 100);
        assert!(!t.playing);
    }
}
