/// Bounded tempo multiplier; MIDI pitches and musical event timestamps stay unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaybackRate(u16);
impl Default for PlaybackRate {
    fn default() -> Self {
        Self(100)
    }
}
impl PlaybackRate {
    pub const MIN: u16 = 25;
    pub const MAX: u16 = 200;
    pub fn new(percent: u16) -> Option<Self> {
        (Self::MIN..=Self::MAX)
            .contains(&percent)
            .then_some(Self(percent))
    }
    pub fn percent(self) -> u16 {
        self.0
    }
    pub fn song_time(self, wall_us: u64) -> u64 {
        ((u128::from(wall_us) * u128::from(self.0) / 100).min(u128::from(u64::MAX))) as u64
    }
    pub fn wall_time(self, song_us: u64) -> u64 {
        ((u128::from(song_us) * 100 / u128::from(self.0)).min(u128::from(u64::MAX))) as u64
    }
}
/// Transport never reads wall time. The platform passes elapsed audio-clock time.
#[derive(Debug, Default)]
pub struct Transport {
    pub position_us: u64,
    pub playing: bool,
    pub duration_us: u64,
    rate: PlaybackRate,
    remainder: u8,
    passage: Option<crate::passage::Passage>,
}
impl Transport {
    pub fn new(duration_us: u64) -> Self {
        Self {
            duration_us,
            ..Self::default()
        }
    }
    pub fn rate(&self) -> PlaybackRate {
        self.rate
    }
    pub fn set_rate(&mut self, rate: PlaybackRate) {
        self.rate = rate;
        self.remainder = 0;
    }
    pub fn passage(&self) -> Option<crate::passage::Passage> {
        self.passage
    }
    pub fn set_passage(&mut self, passage: Option<crate::passage::Passage>) -> bool {
        if passage.is_some_and(|p| {
            crate::passage::Passage::new(p.start_us, p.end_us, self.duration_us).is_none()
        }) {
            return false;
        }
        self.passage = passage;
        self.remainder = 0;
        true
    }
    /// Returns crossed loop boundaries; large frames use bounded arithmetic.
    pub fn advance(&mut self, elapsed_us: u64) -> u64 {
        if self.playing {
            let scaled = u128::from(elapsed_us) * u128::from(self.rate.percent())
                + u128::from(self.remainder);
            self.remainder = (scaled % 100) as u8;
            let elapsed_us = (scaled / 100).min(u128::from(u64::MAX)) as u64;
            if let Some(p) = self.passage {
                let next = u128::from(self.position_us) + u128::from(elapsed_us);
                if next >= u128::from(p.end_us) {
                    let length = u128::from(p.end_us - p.start_us);
                    let beyond = next - u128::from(p.end_us);
                    self.position_us = p.start_us + (beyond % length) as u64;
                    return (1 + beyond / length).min(u128::from(u64::MAX)) as u64;
                }
            }
            self.position_us = self
                .position_us
                .saturating_add(elapsed_us)
                .min(self.duration_us);
            if self.position_us == self.duration_us {
                self.playing = false;
            }
        }
        0
    }
    pub fn seek(&mut self, position_us: u64) {
        self.position_us = if let Some(p) = self.passage {
            position_us.clamp(p.start_us, p.end_us.saturating_sub(1))
        } else {
            position_us.min(self.duration_us)
        };
        self.remainder = 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tempo_scales_without_frame_rounding_drift() {
        for percent in [25, 60, 70, 100, 120, 200] {
            let rate = PlaybackRate::new(percent).unwrap();
            let mut t = Transport::new(u64::MAX);
            t.set_rate(rate);
            t.playing = true;
            for _ in 0..1000 {
                t.advance(1);
            }
            assert_eq!(t.position_us, u64::from(percent) * 10);
            assert_eq!(rate.wall_time(rate.song_time(1_000_000)), 1_000_000);
            let pos = t.position_us;
            t.playing = false;
            t.advance(1_000_000);
            assert_eq!(t.position_us, pos);
            t.set_rate(PlaybackRate::default());
            t.playing = true;
            t.advance(100);
            assert_eq!(t.position_us, pos + 100);
        }
        assert!(PlaybackRate::new(0).is_none());
        assert!(PlaybackRate::new(201).is_none());
        assert_eq!(PlaybackRate::new(25).unwrap().wall_time(u64::MAX), u64::MAX);
    }
    #[test]
    fn loop_keeps_rate_and_handles_many_boundaries() {
        let mut t = Transport::new(1000);
        assert!(t.set_passage(crate::passage::Passage::new(100, 200, 1000)));
        t.seek(100);
        t.playing = true;
        t.set_rate(PlaybackRate::new(70).unwrap());
        assert_eq!(t.advance(100), 0);
        assert_eq!(t.position_us, 170);
        assert_eq!(t.advance(100), 1);
        assert_eq!(t.position_us, 140);
        assert!(t.playing);
        assert_eq!(t.advance(1000), 7);
        assert_eq!(t.position_us, 140);
        t.playing = false;
        assert_eq!(t.advance(u64::MAX), 0);
        assert_eq!(t.position_us, 140);
        t.seek(0);
        assert_eq!(t.position_us, 100);
        t.seek(900);
        assert_eq!(t.position_us, 199);
        assert!(!t.set_passage(Some(crate::passage::Passage {
            start_us: 2,
            end_us: 1
        })));
    }
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
