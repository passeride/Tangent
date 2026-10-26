use tangent_core::piano::PianoLayout;
pub const BLACK_RISE: f32 = 12.;
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Rect {
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}
pub struct Layout {
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub strike: f32,
    pub key_h: f32,
    pub unit: f32,
    pub margin: f32,
    pub progress: Rect,
}
impl Layout {
    pub fn new(width: f32, height: f32, piano: &PianoLayout) -> Self {
        let compact = width < 760.;
        let top = 60.;
        let short = height < 500.;
        let key_h = (height * 0.2).clamp(
            if short { 48. } else { 65. },
            if short { 80. } else { 130. },
        );
        let strike = (height - 34. - key_h).max(top + 20.);
        let margin = if compact { 10. } else { 28. };
        Self {
            width,
            height,
            top,
            strike,
            key_h,
            unit: (width - 2. * margin) / piano.white_count as f32,
            margin,
            progress: Rect {
                x: margin,
                y: strike + key_h + 12.,
                w: width - 2. * margin,
                h: 10.,
            },
        }
    }
    pub fn key(&self, piano: &PianoLayout, key: u8) -> Option<Rect> {
        piano.keys.iter().find(|k| k.key == key).map(|k| Rect {
            x: self.margin + k.x * self.unit,
            y: self.strike - if k.black { BLACK_RISE } else { 0. },
            w: k.width * self.unit,
            h: self.key_h * if k.black { 0.62 } else { 1. } + if k.black { BLACK_RISE } else { 0. },
        })
    }
    pub fn center(&self, piano: &PianoLayout, key: u8) -> Option<f32> {
        self.key(piano, key).map(|r| r.x + r.w / 2.)
    }
    pub fn hit(&self, piano: &PianoLayout, x: f32, y: f32) -> Option<u8> {
        // Raised black-key backs project above the strike rail. Use the same
        // projected rectangles as drawing, always checking black keys first.
        [true, false].into_iter().find_map(|black| {
            piano
                .keys
                .iter()
                .filter(|k| k.black == black)
                .find_map(|k| {
                    self.key(piano, k.key)
                        .filter(|r| r.contains(x, y))
                        .map(|_| k.key)
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_keyboard_keeps_lane_centers_and_black_key_priority() {
        for (first, last) in [(36, 84), (36, 96), (21, 108)] {
            let piano = PianoLayout::new(first, last).unwrap();
            for (width, height) in [(390., 700.), (1280., 420.), (3840., 2160.)] {
                let l = Layout::new(width, height, &piano);
                for key in &piano.keys {
                    let r = l.key(&piano, key.key).unwrap();
                    assert!((l.center(&piano, key.key).unwrap() - (r.x + r.w * 0.5)).abs() < 0.001);
                    let y = if key.black {
                        r.y + 2.
                    } else {
                        l.strike + l.key_h * 0.9
                    };
                    assert_eq!(l.hit(&piano, r.x + r.w * 0.5, y), Some(key.key));
                }
                assert_eq!(l.hit(&piano, -1., l.strike), None);
                assert_eq!(l.hit(&piano, width + 1., l.strike), None);
            }
        }
    }
}
