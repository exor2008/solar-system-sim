pub mod constants;

use bevy::{input::mouse::MouseWheel, prelude::*};
use constants::*;

#[derive(Component, Default)]
pub struct Coord(Vec3);
#[derive(Component, Default)]
pub struct Velocity(Vec3);
#[derive(Component, Default)]
pub struct Mass(f32);
#[derive(Default, Bundle)]
struct BodyBundle {
    pbr: PbrBundle,
    mass: Mass,
    // coord: Coord,
    velocity: Velocity,
    // force: Force,
}

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
        // ..default()
    };

    commands.spawn(sun);
    commands.spawn(earth);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, AU * 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn attraction(time: Res<Time>, mut query: Query<(&Mass, &GlobalTransform, &mut Velocity)>) {
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

        vel1.0 += force1 * time.delta_seconds();

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

pub fn update(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut query {
        transform.translation += vel.0 * TIMESTEP; //* time.delta_seconds();
    }
}

// fn look_at_star(mut camera: Query<&mut Transform, With<Camera>>) {
//     let mut camera = camera.single_mut();
//     let new_rotation = camera
//         .looking_at(Vec3::ZERO, Vec3::Y)
//         .rotation
//         .lerp(camera.rotation, 0.1);
//     camera.rotation = new_rotation;
// }

pub fn scroll_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, With<Camera>)>,
) {
    for (mut transform, _camera) in query.iter_mut() {
        for ev in ev_scroll.read() {
            // Adjust the camera's Z translation based on the scroll delta
            transform.translation.z -= ev.y * AU * 0.2; // Adjust the factor as needed
        }
    }
}
