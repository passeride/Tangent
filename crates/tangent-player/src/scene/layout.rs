use tangent_core::piano::PianoLayout;
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
    pub compact: bool,
}
impl Layout {
    pub fn new(width: f32, height: f32, piano: &PianoLayout) -> Self {
        let compact = width < 760.;
        let top = if compact { 148. } else { 132. };
        let short = height < 500.;
        let key_h = (height * 0.2).clamp(
            if short { 48. } else { 65. },
            if short { 80. } else { 130. },
        );
        let strike = (height - if short { 80. } else { 100. } - key_h).max(top + 20.);
        let margin = if compact { 10. } else { 28. };
        Self {
            width,
            height,
            top,
            strike,
            key_h,
            unit: (width - 2. * margin) / piano.white_count as f32,
            margin,
            compact,
            progress: Rect {
                x: margin,
                y: strike + key_h + 14.,
                w: width - 2. * margin,
                h: 14.,
            },
        }
    }
    pub fn key(&self, piano: &PianoLayout, key: u8) -> Option<Rect> {
        piano.keys.iter().find(|k| k.key == key).map(|k| Rect {
            x: self.margin + k.x * self.unit,
            y: self.strike,
            w: k.width * self.unit,
            h: self.key_h * if k.black { 0.62 } else { 1. },
        })
    }
    pub fn center(&self, piano: &PianoLayout, key: u8) -> Option<f32> {
        self.key(piano, key).map(|r| r.x + r.w / 2.)
    }
    pub fn hit(&self, piano: &PianoLayout, x: f32, y: f32) -> Option<u8> {
        piano.hit(
            (x - self.margin) / self.unit,
            (y - self.strike) / self.key_h,
        )
    }
}
