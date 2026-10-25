//! Shared judgement and VFX presets. All render backends consume the same events.
//! Player input alone awards streaks: backing-track notes never do.
use super::*;
/// Logical-pixel scale shared by Hanabi and WebGL2. Bevy reports window sizes
/// in logical pixels, so OS DPI scaling is already accounted for.
pub(super) fn viewport_scale(width: f32, height: f32) -> f32 {
    (width / 1280.0).min(height / 800.0).clamp(0.65, 3.5)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Grade {
    Correct,
    Wrong,
    Neutral,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(super) enum Preset {
    Correct,
    Wrong,
    FreePlay,
    Streak5,
    Streak10,
    Streak25,
}
impl Preset {
    #[cfg(not(target_arch = "wasm32"))]
    pub const ALL: [Self; 6] = [
        Self::Correct,
        Self::Wrong,
        Self::FreePlay,
        Self::Streak5,
        Self::Streak10,
        Self::Streak25,
    ];
    #[cfg(not(target_arch = "wasm32"))]
    pub fn name(self) -> &'static str {
        match self {
            Self::Correct => "correct",
            Self::Wrong => "wrong",
            Self::FreePlay => "free-play",
            Self::Streak5 => "streak-5",
            Self::Streak10 => "streak-10",
            Self::Streak25 => "streak-25",
        }
    }
    // RGB and emission settings are identical for Hanabi and the WebGL2 fallback.
    pub fn rgb(self) -> [f32; 3] {
        match self {
            Self::Correct => [0.30, 0.95, 0.68],
            Self::Wrong => [1.0, 0.34, 0.38],
            Self::FreePlay => [0.40, 0.68, 0.95],
            Self::Streak5 => [1.0, 0.76, 0.26],
            Self::Streak10 => [0.82, 0.48, 1.0],
            Self::Streak25 => [1.0, 0.94, 0.57],
        }
    }
    pub fn lifetime(self) -> f32 {
        match self {
            Self::Wrong => 0.48,
            Self::FreePlay => 0.68,
            Self::Correct => 0.90,
            Self::Streak5 => 1.15,
            Self::Streak10 => 1.35,
            Self::Streak25 => 1.55,
        }
    }
    pub fn burst_radius(self) -> f32 {
        match self {
            Self::Wrong => 85.0,
            Self::FreePlay => 65.0,
            Self::Correct => 140.0,
            Self::Streak5 => 195.0,
            Self::Streak10 => 270.0,
            Self::Streak25 => 365.0,
        }
    }
    pub fn count(self) -> usize {
        match self {
            Self::Correct => 32,
            Self::Wrong => 22,
            Self::FreePlay => 12,
            Self::Streak5 => 60,
            Self::Streak10 => 96,
            Self::Streak25 => 144,
        }
    }
}
#[derive(Clone, Copy)]
pub(super) struct Burst {
    pub key: u8,
    pub preset: Preset,
}
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
pub(super) struct ScreenBurst {
    pub x: f32,
    pub y: f32,
    pub preset: Preset,
}
#[derive(Default)]
pub(super) struct Feedback {
    streak: usize,
    best: usize,
}
impl Feedback {
    pub fn streak(&self) -> usize {
        self.streak
    }
    pub fn best(&self) -> usize {
        self.best
    }
    pub fn reset(&mut self) {
        self.streak = 0;
    }
    pub fn register(&mut self, grade: Grade) -> Preset {
        match grade {
            Grade::Wrong => {
                self.streak = 0;
                Preset::Wrong
            }
            Grade::Neutral => Preset::FreePlay,
            Grade::Correct => {
                self.streak += 1;
                self.best = self.best.max(self.streak);
                match self.streak {
                    5 => Preset::Streak5,
                    10 => Preset::Streak10,
                    n if n >= 25 && n % 25 == 0 => Preset::Streak25,
                    _ => Preset::Correct,
                }
            }
        }
    }
}
/// A generous hit window for live MIDI; a held note may also be hit mid-note.
/// Percussion tracks never count as piano targets.
pub(super) fn expected_song_hit(song: &Song, position_us: u64, key: u8) -> bool {
    const EARLY_US: u64 = 180_000;
    const LATE_US: u64 = 180_000;
    song.notes.iter().any(|n| {
        n.key == key
            && !song.parts[n.part].percussion
            && n.start_us <= position_us.saturating_add(EARLY_US)
            && n.end_us.saturating_add(LATE_US) >= position_us
    })
}
/// Exact octave in note mode; any inversion of a chord is valid.
pub(super) fn belongs_to_target(practice: &tangent_core::practice::Practice, key: u8) -> bool {
    practice.target().iter().any(|&target| {
        if practice.chords {
            target % 12 == key % 12
        } else {
            target == key
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn responsive_scale_matches_logical_viewport_size() {
        assert_eq!(viewport_scale(1280.0, 800.0), 1.0);
        assert_eq!(viewport_scale(3840.0, 2160.0), 2.7);
        assert_eq!(viewport_scale(1920.0, 1080.0), 1.35);
        assert_eq!(viewport_scale(390.0, 844.0), 0.65);
    }
    #[test]
    fn larger_milestones_have_more_particles_and_reach() {
        for pair in [Preset::Correct, Preset::Streak5, Preset::Streak10].windows(2) {
            assert!(pair[0].count() < pair[1].count());
            assert!(pair[0].burst_radius() < pair[1].burst_radius());
        }
    }
    #[test]
    fn streak_milestones_and_wrong_note_reset() {
        let mut f = Feedback::default();
        assert_eq!(f.register(Grade::Neutral), Preset::FreePlay);
        for _ in 0..4 {
            assert_eq!(f.register(Grade::Correct), Preset::Correct);
        }
        assert_eq!(f.register(Grade::Correct), Preset::Streak5);
        for _ in 0..4 {
            f.register(Grade::Correct);
        }
        assert_eq!(f.register(Grade::Correct), Preset::Streak10);
        for _ in 0..14 {
            f.register(Grade::Correct);
        }
        assert_eq!(f.register(Grade::Correct), Preset::Streak25);
        assert_eq!(f.register(Grade::Wrong), Preset::Wrong);
        assert_eq!(f.streak(), 0);
        assert_eq!(f.best(), 25);
        assert_eq!(f.register(Grade::Correct), Preset::Correct);
    }
    #[test]
    fn chord_membership_accepts_inversions_but_notes_require_octave() {
        let chord = tangent_core::practice::Practice::new(true);
        assert!(belongs_to_target(&chord, 48)); // C3 matches C major target
        assert!(!belongs_to_target(&chord, 61));
        let note = tangent_core::practice::Practice::new(false);
        assert!(belongs_to_target(&note, 60));
        assert!(!belongs_to_target(&note, 48));
    }
    #[test]
    fn song_matches_only_within_timing_window() {
        let song = import::study();
        let n = song
            .notes
            .iter()
            .find(|n| !song.parts[n.part].percussion)
            .unwrap();
        assert!(expected_song_hit(&song, n.start_us, n.key));
        assert!(expected_song_hit(
            &song,
            n.start_us.saturating_sub(100_000),
            n.key
        ));
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub(super) mod gpu {
    use super::*;
    use bevy_hanabi::prelude::*;
    #[derive(Resource)]
    struct Catalog([Handle<EffectAsset>; 6]);
    #[derive(Component)]
    struct TimedEffect(f32);
    pub fn install(app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, flush.after(super::super::draw::render));
    }
    fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let assets = Preset::ALL.map(|preset| effects.add(asset(preset)));
        commands.insert_resource(Catalog(assets));
    }
    fn asset(preset: Preset) -> EffectAsset {
        let writer = ExprWriter::new();
        let lifetime = preset.lifetime();
        let zoom = writer.add_property("vfx_scale", 1.0_f32.into());
        let zoom_value = writer.prop(zoom);
        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
        let init_lifetime =
            SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(lifetime).expr());
        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(5.0).expr(),
            dimension: ShapeDimension::Volume,
        };
        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: (writer.lit(preset.burst_radius() * 1.8) * zoom_value.clone()).expr(),
        };
        let gravity =
            AccelModifier::new((writer.lit(Vec3::new(0., -135., 0.)) * zoom_value.clone()).expr());
        // Initialize size as a particle attribute to support per-instance
        // scaling. SetSizeModifier overrides particle size with a fixed value.
        let init_size = SetAttributeModifier::new(
            Attribute::SIZE,
            (writer.lit(if preset == Preset::Streak25 { 11. } else { 8. }) * zoom_value).expr(),
        );
        let [r, g, b] = preset.rgb();
        let mut gradient = bevy_hanabi::Gradient::new();
        gradient.add_key(0., Vec4::new(1., 1., 1., 1.));
        gradient.add_key(0.10, Vec4::new(r, g, b, 0.98));
        gradient.add_key(0.72, Vec4::new(r, g, b, 0.70));
        gradient.add_key(1., Vec4::new(r, g, b, 0.));
        EffectAsset::new(
            256,
            SpawnerSettings::once((preset.count() as f32).into()),
            writer.finish(),
        )
        .with_name(preset.name())
        .init(init_age)
        .init(init_lifetime)
        .init(init_pos)
        .init(init_vel)
        .init(init_size)
        .update(gravity)
        .render(ColorOverLifetimeModifier::new(gradient))
        .render(ScreenSpaceSizeModifier)
    }
    fn flush(
        mut commands: Commands,
        mut state: ResMut<State>,
        catalog: Res<Catalog>,
        windows: Query<&Window>,
        mut running: Query<(Entity, &mut TimedEffect)>,
        time: Res<Time>,
    ) {
        let dt = time.delta_secs().min(0.05);
        for (entity, mut life) in &mut running {
            life.0 -= dt;
            if life.0 <= 0. || !state.animated_effects() || state.screen != Screen::Session {
                commands.entity(entity).despawn();
            }
        }
        let events = std::mem::take(&mut state.gpu_bursts);
        if !state.animated_effects() || state.screen != Screen::Session {
            return;
        }
        let Ok(window) = windows.single() else { return };
        for event in events.into_iter().take(32) {
            // Bevy's Camera2d uses centered world coordinates; UI uses top-left pixels.
            let x = event.x - window.width() * 0.5;
            let y = window.height() * 0.5 - event.y;
            let mut properties = EffectProperties::default();
            properties.set(
                "vfx_scale",
                (viewport_scale(window.width(), window.height()) * state.effect_scale()).into(),
            );
            commands.spawn((
                ParticleEffect::new(catalog.0[event.preset as usize].clone()),
                properties,
                Transform::from_xyz(x, y, 1.),
                TimedEffect(event.preset.lifetime() + 0.25),
            ));
        }
    }
}
