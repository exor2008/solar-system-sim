pub mod constants;

use bevy::{input::mouse::MouseWheel, math::DVec3, prelude::*};
use constants::*;

#[derive(Component, Default)]
pub struct Coord(DVec3);

#[derive(Component, Default)]
pub struct Velocity(DVec3);

#[derive(Component, Default)]
pub struct Mass(f64);

#[derive(Component, Default)]
pub struct Ordinal(usize);

#[derive(Component, Default)]
pub struct TargetZ(f64);

#[derive(Resource, Default)]
pub struct Lerp(f32);

// #[derive(Component, Default)]
// pub struct Radius(f64);

#[derive(Default, Bundle)]
struct BodyBundle {
    pbr: PbrBundle,
    mass: Mass,
    velocity: Velocity,
    // radius: Radius,
    coord: Coord,
}

#[derive(Default, Component)]
struct Star;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<GizmoConfig>,
) {
    // Sun
    let sun = BodyBundle {
        pbr: PbrBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (SUN_RADIUS * SCALE) as f32,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                emissive: (Color::ORANGE_RED * 2.),
                ..default()
            }),
            ..default()
        },
        mass: Mass(SUN_MASS),
        velocity: Velocity(DVec3::ZERO),
        // radius: Radius(SUN_RADIUS),
        coord: Coord(DVec3::ZERO), // ..default()
    };

    // Earth
    let earth: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((AU * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (EARTH_RADIUS * SCALE) as f32,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            ..default()
        },
        mass: Mass(EARTH_MASS),
        velocity: Velocity(DVec3::new(0.0, EARTH_VEL, 0.0)),
        // radius: Radius(EARTH_RADIUS),
        coord: Coord(DVec3::new(AU, 0.0, 0.0)),
        // ..default()
    };

    // Moon
    let moon: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz(((AU + MOON_SHIFT) * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (MOON_RADIUS * SCALE) as f32,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            ..default()
        },
        mass: Mass(MOON_MASS),
        velocity: Velocity(DVec3::new(0.0, MOON_VEL + EARTH_VEL, 0.0)),
        // radius: Radius(EARTH_RADIUS),
        coord: Coord(DVec3::new(AU + MOON_SHIFT, 0.0, 0.0)),
        // ..default()
    };

    commands.spawn((sun, Star));
    commands.spawn(earth);
    commands.spawn(moon);

    let position =
        Transform::from_xyz(0.0, 0.0, (AU * 3.0 * SCALE) as f32).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform: position,
            ..default()
        },
        Ordinal(0),
        TargetZ(AU * 3.0),
    ));

    config.line_width = 3.0;

    commands.init_resource::<Lerp>();
}

pub fn attraction(_time: Res<Time>, mut query: Query<(&Mass, &mut Velocity, &Coord)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), mut vel1, coord1), (Mass(m2), mut vel2, coord2)]) = iter.fetch_next()
    {
        let diff1 = coord2.0 - coord1.0;
        let distance = diff1.length();

        let phi1 = diff1.y.atan2(diff1.x);
        let theta1 = (diff1.z / distance).acos();

        let force1 = (G * m2) / (distance * distance);

        let x1 = force1 * theta1.sin() * phi1.cos();
        let y1 = force1 * theta1.sin() * phi1.sin();
        let z1 = force1 * theta1.cos();

        let force1 = DVec3::new(x1, y1, z1);

        vel1.0 += force1 * TIMESTEP; //time.delta_seconds();

        let diff2 = coord1.0 - coord2.0;

        let phi2 = diff2.y.atan2(diff2.x);
        let theta2 = (diff2.z / distance).acos();

        let force2 = (G * m1) / (distance * distance);

        let x2 = force2 * theta2.sin() * phi2.cos();
        let y2 = force2 * theta2.sin() * phi2.sin();
        let z2 = force2 * theta2.cos();

        let force2 = DVec3::new(x2, y2, z2);

        vel2.0 += force2 * TIMESTEP; //* time.delta_seconds();
    }
}

pub fn update_lerp(mut lerp: ResMut<Lerp>) {
    lerp.0 += 0.05;
    lerp.0 = lerp.0.min(1.0);
    // println!("{}", lerp.0);
}

pub fn update(mut bodies: Query<(&Velocity, &mut Transform, &mut Coord)>) {
    for (vel, mut transform, mut coord) in &mut bodies {
        coord.0 += vel.0 * TIMESTEP; //* time.delta_seconds();
        transform.translation = (coord.0 * SCALE).as_vec3();
    }
}

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    bodies: Query<&Coord>,
    camera: Query<&TargetZ, With<Camera>>,
) {
    let target_z = camera.single();
    for coord in &bodies {
        let r = (target_z.0 * 0.01 * SCALE) as f32;
        gizmos.circle_2d((coord.0 * SCALE).as_vec3().xy(), r, Color::RED);
    }
}

pub fn look_at_target(
    mut camera: Query<(&Ordinal, &TargetZ, &mut Transform, With<Camera>)>,
    bodies: Query<(&Coord, With<Mass>)>,
    lerp: Res<Lerp>,
) {
    let (ordinal, target_z, mut camera_transform, _) = camera.single_mut();

    let (coord, _) = bodies.iter().nth(ordinal.0).unwrap();

    let coord = (coord.0 * SCALE).as_vec3();
    let new_target = Vec3::new(coord.x, coord.y, (target_z.0 * SCALE) as f32);

    let new_translation = camera_transform.translation.lerp(new_target, lerp.0);
    camera_transform.translation = new_translation;
}

pub fn scroll_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera: Query<&mut TargetZ, With<Camera>>,
) {
    let mut target_z = camera.single_mut();

    for ev in ev_scroll.read() {
        target_z.0 -= ev.y as f64 * (target_z.0) / 4.0;
    }
}

pub fn switch_track(
    keyboard_input: Res<Input<KeyCode>>,
    bodies: Query<(&GlobalTransform, With<Mass>)>,
    mut camera: Query<&mut Ordinal, With<Camera>>,
    mut lerp: ResMut<Lerp>,
) {
    let mut ordinal = camera.single_mut();
    let count = bodies.iter().count();

    if keyboard_input.just_pressed(KeyCode::Left) {
        lerp.0 = 0.1;
        ordinal.0 = if ordinal.0 == 0 {
            count - 1
        } else {
            ordinal.0 - 1
        }
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        lerp.0 = 0.1;
        ordinal.0 = if ordinal.0 == count - 1 {
            0
        } else {
            ordinal.0 + 1
        }
    }
}
