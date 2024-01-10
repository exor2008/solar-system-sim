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

#[derive(Component, Default)]
pub struct Trajectory(Vec<DVec3>);

#[derive(Resource, Default)]
pub struct Lerp(f32);

#[derive(Default, Bundle)]
struct BodyBundle {
    pbr: PbrBundle,
    mass: Mass,
    velocity: Velocity,
    coord: Coord,
    trajectory: Trajectory,
}

#[derive(Default, Component)]
struct Star;

#[derive(Component)]
pub struct Label {
    entity: Entity,
    shift: f32,
    text: String,
}

#[derive(Component, Default)]
pub struct Labled;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
                    subdivisions: 10,
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
        coord: Coord(DVec3::ZERO),
        ..default()
    };

    // Mercury
    let mercury: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((MERCURY_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (MERCURY_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..default()
        },
        mass: Mass(MERCURY_MASS),
        velocity: Velocity(DVec3::new(0.0, MERCURY_VEL, 0.0)),
        coord: Coord(DVec3::new(MERCURY_DISTANCE, 0.0, 0.0)),
        ..default()
    };

    // Venus
    let venus: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((VENUS_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (VENUS_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.52, 0.0, 1.0).into()),
            ..default()
        },
        mass: Mass(VENUS_MASS),
        velocity: Velocity(DVec3::new(0.0, VENUS_VEL, 0.0)),
        coord: Coord(DVec3::new(VENUS_DISTANCE, 0.0, 0.0)),
        ..default()
    };

    // Earth
    let earth: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((EARTH_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (EARTH_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            ..default()
        },
        mass: Mass(EARTH_MASS),
        velocity: Velocity(DVec3::new(0.0, EARTH_VEL, 0.0)),
        coord: Coord(DVec3::new(EARTH_DISTANCE, 0.0, 0.0)),
        ..default()
    };

    // Moon
    let moon: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz(
                ((EARTH_DISTANCE + MOON_DISTANCE) * SCALE) as f32,
                0.0,
                0.0,
            ),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (MOON_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            ..default()
        },
        mass: Mass(MOON_MASS),
        velocity: Velocity(DVec3::new(0.0, EARTH_VEL + MOON_VEL, 0.0)),
        coord: Coord(DVec3::new(EARTH_DISTANCE + MOON_DISTANCE, 0.0, 0.0)),
        ..default()
    };

    // Mars
    let mars: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((MARS_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (MARS_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.83, 0.35, 0.21).into()),
            ..default()
        },
        mass: Mass(MARS_MASS),
        velocity: Velocity(DVec3::new(0.0, MARS_VEL, 0.0)),
        coord: Coord(DVec3::new(MARS_DISTANCE, 0.0, 0.0)),
        ..default()
    };

    let sun = commands.spawn((sun, Star)).id();
    let mercury = commands.spawn(mercury).id();
    let venus = commands.spawn(venus).id();
    let earth = commands.spawn(earth).id();
    let moon = commands.spawn(moon).id();
    let mars = commands.spawn(mars).id();

    // Camera
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

    // Label
    let font = asset_server.load("PressStart2P-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    let mut label = |entity: Entity, label: &str, coef: f32| {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                },
                Label {
                    shift: coef,
                    text: label.to_string(),
                    entity,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section("", text_style.clone())
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::ZERO,
                            ..default()
                        })
                        .with_no_wrap(),
                    Labled,
                ));
            });
    };

    label(sun, "Sun", 6.0);
    label(mercury, "Mercury", 0.02);
    label(venus, "Venus", 0.05);
    label(earth, "Earth", 0.05);
    label(moon, "Moon", 0.01);
    label(mars, "Mars", 0.03);
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

        vel1.0 += force1 * TIMESTEP;

        let diff2 = coord1.0 - coord2.0;

        let phi2 = diff2.y.atan2(diff2.x);
        let theta2 = (diff2.z / distance).acos();

        let force2 = (G * m1) / (distance * distance);

        let x2 = force2 * theta2.sin() * phi2.cos();
        let y2 = force2 * theta2.sin() * phi2.sin();
        let z2 = force2 * theta2.cos();

        let force2 = DVec3::new(x2, y2, z2);

        vel2.0 += force2 * TIMESTEP;
    }
}

pub fn update_lerp(mut lerp: ResMut<Lerp>) {
    lerp.0 += 0.05;
    lerp.0 = lerp.0.min(1.0);
}

pub fn update_position(
    mut bodies: Query<(&Velocity, &mut Transform, &mut Coord, &mut Trajectory)>,
) {
    for (vel, mut transform, mut coord, mut trajectory) in &mut bodies {
        coord.0 += vel.0 * TIMESTEP; //* time.delta_seconds();
        transform.translation = (coord.0 * SCALE).as_vec3();
        trajectory.0.push(coord.0);
    }
}

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    bodies: Query<(&Coord, &Trajectory)>,
    camera: Query<&TargetZ, With<Camera>>,
) {
    let target_z = camera.single();
    for (coord, trajectory) in &bodies {
        let r = (target_z.0 * 0.01 * SCALE) as f32;
        gizmos.circle_2d((coord.0 * SCALE).as_vec3().xy(), r, Color::RED);

        for coords in trajectory.0.windows(2) {
            gizmos.line(
                (coords[0] * SCALE).as_vec3(),
                (coords[1] * SCALE).as_vec3(),
                Color::GRAY,
            );
        }
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

pub fn update_labels(
    mut labels: Query<(&mut Style, &Label)>,
    bodies: Query<&GlobalTransform>,
    mut camera: Query<(&mut Camera, &GlobalTransform)>,
    mut label_text: Query<&mut Text, With<Labled>>,
) {
    let (camera, camera_global_transform) = camera.single_mut();

    for ((mut style, label), mut text) in &mut labels.iter_mut().zip(&mut label_text) {
        let world_position = bodies.get(label.entity).unwrap().translation();
        let dist = world_position.length();
        let world_position = world_position + Vec3::new(label.shift, -label.shift, 0.0);

        if let Some(viewport_position) =
            camera.world_to_viewport(camera_global_transform, world_position)
        {
            style.top = Val::Px(viewport_position.y);
            style.left = Val::Px(viewport_position.x);

            text.sections[0].value =
                format!("{}\n{:.4} AU", label.text, dist / (AU * SCALE) as f32);
        }
    }
}
