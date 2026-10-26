use serde::Serialize;
#[derive(Debug, Clone, Copy, Serialize)]
pub struct KeyGeometry {
    pub key: u8,
    pub black: bool,
    /// Piano coordinates are in white-key widths, independent of pixels/theme.
    pub x: f32,
    pub width: f32,
}
#[derive(Debug, Clone, Serialize)]
pub struct PianoLayout {
    pub first: u8,
    pub last: u8,
    pub white_count: usize,
    pub keys: Vec<KeyGeometry>,
}
pub fn black(key: u8) -> bool {
    matches!(key % 12, 1 | 3 | 6 | 8 | 10)
}
/// Smallest supported fixed keyboard that contains the selected pitches.
/// Values beyond a physical 88-key piano remain visible through UI warnings.
pub fn fit_range(keys: impl IntoIterator<Item = u8>) -> (u8, u8) {
    let (low, high) = keys
        .into_iter()
        .fold((u8::MAX, 0), |(low, high), k| (low.min(k), high.max(k)));
    [(36, 84), (36, 96), (21, 108)]
        .into_iter()
        .find(|(a, b)| low >= *a && high <= *b)
        .unwrap_or((21, 108))
}
impl PianoLayout {
    /// Require white endpoints so neither edge contains a clipped accidental.
    pub fn new(first: u8, last: u8) -> Option<Self> {
        if first > last || last > 127 || black(first) || black(last) {
            return None;
        }
        let mut white_count = 0;
        let keys = (first..=last)
            .map(|key| {
                let is_black = black(key);
                let x = if is_black {
                    white_count as f32 - 0.32
                } else {
                    let x = white_count as f32;
                    white_count += 1;
                    x
                };
                KeyGeometry {
                    key,
                    black: is_black,
                    x,
                    width: if is_black { 0.64 } else { 1.0 },
                }
            })
            .collect();
        Some(Self {
            first,
            last,
            white_count,
            keys,
        })
    }
    /// Test black keys first; shared hit geometry prevents accidental white hits.
    pub fn hit(&self, x: f32, depth: f32) -> Option<u8> {
        if !(0.0..=1.0).contains(&depth) {
            return None;
        }
        self.keys
            .iter()
            .filter(|k| !k.black || depth < 0.62)
            .filter(|k| x >= k.x && x < k.x + k.width)
            .max_by_key(|k| k.black)
            .map(|k| k.key)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fitting_uses_supported_ranges_without_hiding_extremes() {
        assert_eq!(fit_range([60, 72]), (36, 84));
        assert_eq!(fit_range([60, 90]), (36, 96));
        assert_eq!(fit_range([24, 100]), (21, 108));
        assert_eq!(fit_range([0, 127]), (21, 108));
    }
    #[test]
    fn full_piano_and_black_key_priority() {
        let p = PianoLayout::new(21, 108).unwrap();
        assert_eq!(p.white_count, 52);
        assert_eq!(p.keys.len(), 88);
        let k = p.keys.iter().find(|k| k.key == 61).unwrap();
        assert_eq!(p.hit(k.x + 0.1, 0.2), Some(61));
        assert_eq!(p.hit(k.x + 0.1, 0.9), Some(60));
        assert_eq!(p.hit(-1.0, 0.2), None);
    }
}
