pub mod constants;

use bevy::{input::mouse::MouseWheel, prelude::*};
use constants::*;

#[derive(Component, Default)]
pub struct Coord(Vec3);

#[derive(Component, Default)]
pub struct Velocity(Vec3);

#[derive(Component, Default)]
pub struct Mass(f32);

#[derive(Component, Default)]
pub struct Ordinal(usize);

#[derive(Component, Default)]
pub struct TargetZ(f32);

#[derive(Component, Default)]
pub struct Radius(f32);

#[derive(Default, Bundle)]
struct BodyBundle {
    pbr: PbrBundle,
    mass: Mass,
    velocity: Velocity,
    radius: Radius,
}

#[derive(Default, Component)]
struct Star;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                    radius: SUN_RADIUS,
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
        velocity: Velocity(Vec3::ZERO),
        radius: Radius(SUN_RADIUS),
        // ..default()
    };

    // Earth
    let earth: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz(AU, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: EARTH_RADIUS,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            ..default()
        },
        mass: Mass(EARTH_MASS),
        velocity: Velocity(Vec3::new(0.0, EARTH_VEL, 0.0)),
        radius: Radius(EARTH_RADIUS),
        // ..default()
    };

    commands.spawn((sun, Star));
    commands.spawn(earth);

    let position = Transform::from_xyz(0.0, 0.0, AU * 3.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform: position,
            ..default()
        },
        Ordinal(0),
        TargetZ(AU * 3.0),
    ));
}

pub fn attraction(_time: Res<Time>, mut query: Query<(&Mass, &GlobalTransform, &mut Velocity)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut vel1), (Mass(m2), transform2, mut vel2)]) =
        iter.fetch_next()
    {
        let diff1 = transform2.translation() - transform1.translation();
        let distance = diff1.length();

        let phi1 = diff1.y.atan2(diff1.x);
        let theta1 = (diff1.z / distance).acos();

        let force1 = (G * m2) / (distance * distance);

        let x1 = force1 * theta1.sin() * phi1.cos();
        let y1 = force1 * theta1.sin() * phi1.sin();
        let z1 = force1 * theta1.cos();

        let force1 = Vec3::new(x1, y1, z1);

        vel1.0 += force1 * TIMESTEP; //time.delta_seconds();

        let diff2 = transform1.translation() - transform2.translation();

        let phi2 = diff2.y.atan2(diff2.x);
        let theta2 = (diff2.z / distance).acos();

        let force2 = (G * m1) / (distance * distance);

        let x2 = force2 * theta2.sin() * phi2.cos();
        let y2 = force2 * theta2.sin() * phi2.sin();
        let z2 = force2 * theta2.cos();

        let force2 = Vec3::new(x2, y2, z2);

        vel2.0 += force2 * TIMESTEP; //* time.delta_seconds();
    }
}

pub fn update(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut query {
        transform.translation += vel.0 * TIMESTEP; //* time.delta_seconds();
    }
}

pub fn look_at_target(
    mut camera: Query<(&Ordinal, &TargetZ, &mut Transform, With<Camera>), With<Camera>>,
    bodies: Query<(&GlobalTransform, With<Mass>)>,
) {
    let (ordinal, target_z, mut camera_transform, _) = camera.single_mut();

    let (body_transform, _) = bodies.iter().nth(ordinal.0).unwrap();

    let new_target = Vec3::new(
        body_transform.translation().x,
        body_transform.translation().y,
        target_z.0,
    );

    let new_translation = camera_transform.translation.lerp(new_target, 0.2);
    camera_transform.translation = new_target;
}

pub fn scroll_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera: Query<(&mut TargetZ, With<Camera>)>,
) {
    let (mut target_z, _) = camera.single_mut();

    for ev in ev_scroll.read() {
        target_z.0 -= ev.y * (target_z.0) / 4.0;
    }
}

pub fn switch_track(
    keyboard_input: Res<Input<KeyCode>>,
    bodies: Query<(&GlobalTransform, With<Mass>)>,
    mut camera: Query<&mut Ordinal, With<Camera>>,
) {
    let mut ordinal = camera.single_mut();
    let count = bodies.iter().count();

    if keyboard_input.just_pressed(KeyCode::Left) {
        ordinal.0 = if ordinal.0 == 0 {
            count - 1
        } else {
            ordinal.0 - 1
        }
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        ordinal.0 = if ordinal.0 == count - 1 {
            0
        } else {
            ordinal.0 + 1
        }
    }
}
