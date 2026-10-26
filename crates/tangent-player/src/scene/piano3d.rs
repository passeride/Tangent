//! Procedural, lit piano rendered into the existing Bevy UI surface.
//! Screen-space world units preserve the musical lane/hit-test contract.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    camera::{RenderTarget, ScalingMode, visibility::RenderLayers},
    mesh::PrimitiveTopology,
    render::render_resource::{Extent3d, TextureFormat},
};

const BACK: f32 = 16.;
const FRONT: f32 = 8.;
const DEPTH_PROJECTION: f32 = 0.8;

#[derive(Resource)]
pub(super) struct PianoScene {
    pub image: Handle<Image>,
    size: UVec2,
    range: (u8, u8),
    key_mesh: Handle<Mesh>,
    box_mesh: Handle<Mesh>,
    case_material: Handle<StandardMaterial>,
    rail_material: Handle<StandardMaterial>,
    owned_materials: Vec<Handle<StandardMaterial>>,
}
#[derive(Component)]
struct PianoCamera;
#[derive(Component)]
struct Part;
#[derive(Component)]
struct Key {
    note: u8,
    black: bool,
    depression: f32,
    surface: Handle<StandardMaterial>,
    edge: Handle<StandardMaterial>,
}
#[derive(Component)]
struct Case(u8);
#[derive(Component)]
struct Rail;

pub(super) fn rect(l: &Layout) -> layout::Rect {
    layout::Rect {
        x: 0.,
        y: l.strike - BACK,
        w: l.width,
        h: l.key_h + BACK + FRONT,
    }
}

pub(super) fn install(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, sync.after(controls::input).before(draw::render));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.add(Image::new_target_texture(
        1,
        1,
        TextureFormat::Rgba8UnormSrgb,
        None,
    ));
    commands.spawn((
        PianoCamera,
        Camera3d::default(),
        Camera {
            order: -1,
            is_active: false,
            ..default()
        },
        RenderTarget::Image(image.clone().into()),
        Msaa::Off,
        Projection::Orthographic(OrthographicProjection {
            far: 4000.,
            ..OrthographicProjection::default_3d()
        }),
        RenderLayers::layer(1),
    ));
    // Two broad lights give ivory highlights and dark key sidewalls without
    // shadow maps, HDR targets or postprocessing requirements on WebGL2.
    for (position, illuminance) in [
        (Vec3::new(-1., 3., 4.), 8500.),
        (Vec3::new(2., 2., -3.), 3200.),
    ] {
        commands.spawn((
            DirectionalLight {
                illuminance,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::layer(1),
        ));
    }
    commands.insert_resource(PianoScene {
        image,
        size: UVec2::ZERO,
        range: (0, 0),
        key_mesh: meshes.add(beveled_key()),
        box_mesh: meshes.add(Cuboid::default()),
        case_material: materials.add(StandardMaterial {
            metallic: 0.28,
            perceptual_roughness: 0.25,
            ..default()
        }),
        rail_material: materials.add(StandardMaterial {
            unlit: true,
            ..default()
        }),
        owned_materials: vec![],
    });
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn sync(
    mut commands: Commands,
    state: Res<State>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut scene: ResMut<PianoScene>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    parts: Query<Entity, With<Part>>,
    mut cameras: Query<(&mut Camera, &mut Projection, &mut Transform), With<PianoCamera>>,
    mut keys: Query<(&mut Key, &mut Transform), Without<PianoCamera>>,
    mut cases: Query<
        (&Case, &mut Transform),
        (
            With<Case>,
            Without<Key>,
            Without<PianoCamera>,
            Without<Rail>,
        ),
    >,
    mut rails: Query<
        &mut Transform,
        (
            With<Rail>,
            Without<Key>,
            Without<PianoCamera>,
            Without<Case>,
        ),
    >,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mut camera, mut projection, mut camera_transform)) = cameras.single_mut() else {
        return;
    };
    camera.is_active = state.screen == Screen::Session;
    if !camera.is_active {
        return;
    }
    let l = Layout::new(window.width(), window.height(), &state.piano);
    let r = rect(&l);
    let t = personal::palette(&state);
    camera.clear_color = t.bg.into();
    // Preserve the aspect ratio when limiting the render target on HiDPI/4K.
    let density = window.scale_factor().min(2.).min(4096. / r.w.max(1.));
    let size = UVec2::new(
        (r.w * density).round().max(1.) as u32,
        (r.h * density).round().max(1.) as u32,
    );
    if size != scene.size {
        if let Some(mut image) = images.get_mut(&scene.image) {
            image.resize(Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            });
        }
        scene.size = size;
    }
    *projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: r.h,
        },
        far: 4000.,
        ..OrthographicProjection::default_3d()
    });
    let target = Vec3::new(r.w * 0.5, 0., (r.h * 0.5 - BACK) / DEPTH_PROJECTION);
    *camera_transform =
        Transform::from_translation(target + Vec3::new(0., 800., 600.)).looking_at(target, Vec3::Y);
    if let Some(mut m) = materials.get_mut(&scene.case_material) {
        m.base_color = t.panel;
    }
    if let Some(mut m) = materials.get_mut(&scene.rail_material) {
        m.base_color = t.colors[0];
        m.emissive = LinearRgba::from(t.colors[0]) * 0.3;
    }
    let range = (
        state.piano.keys.first().unwrap().key,
        state.piano.keys.last().unwrap().key,
    );
    if range != scene.range {
        for entity in &parts {
            commands.entity(entity).despawn();
        }
        // Explicitly release per-key assets when switching 49/61/88-key ranges.
        for handle in scene.owned_materials.drain(..) {
            materials.remove(handle.id());
        }
        for k in &state.piano.keys {
            let surface = materials.add(StandardMaterial {
                perceptual_roughness: if k.black { 0.24 } else { 0.38 },
                metallic: if k.black { 0.12 } else { 0.02 },
                ..default()
            });
            let edge = materials.add(StandardMaterial {
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            scene
                .owned_materials
                .extend([surface.clone(), edge.clone()]);
            let entity = commands
                .spawn((
                    Part,
                    Key {
                        note: k.key,
                        black: k.black,
                        depression: 0.,
                        surface: surface.clone(),
                        edge: edge.clone(),
                    },
                    Mesh3d(scene.key_mesh.clone()),
                    MeshMaterial3d(surface),
                    Transform::default(),
                    RenderLayers::layer(1),
                ))
                .id();
            // Emissive edge rails belong to the moving key, so perspective,
            // black-key occlusion and depression all stay physically coherent.
            commands.entity(entity).with_children(|parent| {
                for (p, s) in [
                    (Vec3::new(-0.455, 0.008, 0.5), Vec3::new(0.018, 0.012, 0.94)),
                    (Vec3::new(0.455, 0.008, 0.5), Vec3::new(0.018, 0.012, 0.94)),
                    (Vec3::new(0., 0.008, 0.025), Vec3::new(0.92, 0.012, 0.012)),
                    (Vec3::new(0., 0.008, 0.975), Vec3::new(0.92, 0.012, 0.012)),
                ] {
                    parent.spawn((
                        Mesh3d(scene.box_mesh.clone()),
                        MeshMaterial3d(edge.clone()),
                        Transform::from_translation(p).with_scale(s),
                        RenderLayers::layer(1),
                    ));
                }
            });
        }
        for kind in 0..5 {
            let rail = kind == 4;
            let material = if rail {
                scene.rail_material.clone()
            } else {
                scene.case_material.clone()
            };
            let mut e = commands.spawn((
                Part,
                Mesh3d(scene.box_mesh.clone()),
                MeshMaterial3d(material),
                Transform::default(),
                RenderLayers::layer(1),
            ));
            if rail {
                e.insert(Rail);
            } else {
                e.insert(Case(kind));
            }
        }
        scene.range = range;
    }
    let depth = (l.key_h - 5.4) / DEPTH_PROJECTION;
    for (case, mut transform) in &mut cases {
        let (center, size) = match case.0 {
            0 => (
                Vec3::new(l.width * 0.5, -9., -3.),
                Vec3::new(l.width - l.margin, 20., 22.),
            ),
            1 => (
                Vec3::new(l.width * 0.5, -12., depth * 0.5),
                Vec3::new(l.width - 2. * l.margin + 12., 7., depth + 3.),
            ),
            2 => (
                Vec3::new(l.margin - 4., -5., depth * 0.5),
                Vec3::new(7., 14., depth + 10.),
            ),
            _ => (
                Vec3::new(l.width - l.margin + 4., -5., depth * 0.5),
                Vec3::new(7., 14., depth + 10.),
            ),
        };
        *transform = Transform::from_translation(center).with_scale(size);
    }
    for mut transform in &mut rails {
        *transform = Transform::from_xyz(l.width * 0.5, 1., -2.).with_scale(Vec3::new(
            l.width - 2. * l.margin,
            1.2,
            1.8,
        ));
    }
    let expected = draw::expected_keys(&state);
    let cues = cue_colors(&state, t);
    let held: BTreeSet<_> = state.held.values().copied().collect();
    let ease = 1. - (-time.delta_secs().min(0.1) * 30.).exp();
    for (mut key, mut transform) in &mut keys {
        let Some(r) = l.key(&state.piano, key.note) else {
            continue;
        };
        let down = held.contains(&key.note);
        let goal = if down { 1. } else { 0. };
        key.depression = if state.reduced_motion {
            goal
        } else {
            key.depression + (goal - key.depression) * ease
        };
        let height = if key.black { 12. } else { 9. };
        let elevation = if key.black { 10. } else { 0. };
        let length = (r.h + r.y - l.strike - 0.6 * height + 0.6 * elevation) / DEPTH_PROJECTION;
        *transform = Transform::from_xyz(r.x + r.w * 0.5, elevation, 0.)
            .with_rotation(Quat::from_rotation_x(key.depression * 3. / length))
            .with_scale(Vec3::new(
                (r.w - (l.unit * 0.06).max(0.7)).max(1.),
                height,
                length,
            ));
        let glow = cues.get(&key.note).copied().unwrap_or(t.colors[0]);
        if let Some(mut m) = materials.get_mut(&key.surface) {
            m.base_color = if down {
                t.colors[0]
            } else if key.black {
                t.black
            } else {
                t.white
            };
            m.emissive = if down {
                LinearRgba::from(t.colors[0]) * 0.28
            } else {
                LinearRgba::BLACK
            };
        }
        if let Some(mut m) = materials.get_mut(&key.edge) {
            m.base_color = glow.with_alpha(if expected.contains(&key.note) {
                0.95
            } else {
                0.
            });
            m.emissive = LinearRgba::from(glow)
                * if expected.contains(&key.note) {
                    1.5
                } else {
                    0.
                };
        }
    }
}

pub(super) fn cue_colors(state: &State, t: theme::Theme) -> BTreeMap<u8, Color> {
    let mut colors = BTreeMap::new();
    if state.mode == Mode::Play {
        let now = state.transport.position_us;
        for n in state
            .song
            .notes
            .iter()
            .take_while(|n| n.start_us <= now)
            .filter(|n| {
                n.end_us > now
                    && !state.song.parts[n.part].percussion
                    && passage::context(state).includes(n)
            })
        {
            colors
                .entry(n.key)
                .or_insert_with(|| personal::note_color(state, n.id, t));
        }
    }
    colors
}

/// A chamfered solid, pivoted at the back of its top surface. Authored here;
/// no external piano model, texture, shader or additional library is required.
fn beveled_key() -> Mesh {
    let rings = [
        (-1., 0.5, 0., 1.),
        (-0.16, 0.5, 0., 1.),
        (0., 0.46, 0.015, 0.985),
    ];
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut face = |points: [Vec3; 4]| {
        let normal = (points[1] - points[0])
            .cross(points[2] - points[0])
            .normalize();
        for i in [0, 1, 2, 0, 2, 3] {
            positions.push(points[i].to_array());
            normals.push(normal.to_array());
        }
    };
    let ring = |(y, x, back, front): (f32, f32, f32, f32)| {
        [
            Vec3::new(-x, y, back),
            Vec3::new(-x, y, front),
            Vec3::new(x, y, front),
            Vec3::new(x, y, back),
        ]
    };
    face(ring(rings[2]));
    let bottom = ring(rings[0]);
    face([bottom[3], bottom[2], bottom[1], bottom[0]]);
    for j in 0..2 {
        let a = ring(rings[j]);
        let b = ring(rings[j + 1]);
        for i in 0..4 {
            let next = (i + 1) % 4;
            face([a[i], a[next], b[next], b[i]]);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}
