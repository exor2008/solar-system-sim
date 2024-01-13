pub mod constants;

use bevy::{math::DVec3, prelude::*};
use bevy_panorbit_camera::PanOrbitCamera;
use constants::*;

#[derive(Component, Default)]
pub struct Coord(DVec3);

#[derive(Component, Default)]
pub struct Velocity(Vec3);

#[derive(Component, Default)]
pub struct Mass(f64);

#[derive(Component, Default)]
pub struct Ordinal(usize);

#[derive(Component, Default)]
pub struct Trajectory(Vec<DVec3>);

#[derive(Component, Default)]
pub struct CircleSize(f32);

#[derive(Resource, Default)]
pub struct PanSoft(f32);

#[derive(Default, Bundle)]
struct BodyBundle {
    pbr: PbrBundle,
    mass: Mass,
    velocity: Velocity,
    coord: Coord,
    trajectory: Trajectory,
    circle_size: CircleSize,
}

#[derive(Default, Component)]
struct Star;

#[derive(Component)]
pub struct Label {
    entity: Entity,
    shift: f32,
    text: String,
    threshold: f32,
}

#[derive(Component, Default)]
pub struct Labled;

pub fn setup(
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
        velocity: Velocity(Vec3::ZERO),
        coord: Coord(DVec3::ZERO),
        circle_size: CircleSize(0.015),
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
        velocity: Velocity(
            Quat::from_rotation_x(MERCURY_INCLINATION) * Vec3::new(0.0, MERCURY_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(MERCURY_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
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
        velocity: Velocity(
            Quat::from_rotation_x(VENUS_INCLINATION) * Vec3::new(0.0, VENUS_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(VENUS_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
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
        velocity: Velocity(
            Quat::from_rotation_x(EARTH_INCLINATION) * Vec3::new(0.0, EARTH_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(EARTH_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
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
        velocity: Velocity(
            Quat::from_rotation_x(EARTH_INCLINATION) * Vec3::new(0.0, EARTH_VEL, 0.0)
                + Quat::from_rotation_x(MOON_INCLINATION) * Vec3::new(0.0, MOON_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(EARTH_DISTANCE + MOON_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.0075),
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
        velocity: Velocity(Quat::from_rotation_x(MARS_INCLINATION) * Vec3::new(0.0, MARS_VEL, 0.0)),
        coord: Coord(DVec3::new(MARS_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
        ..default()
    };

    // Ceres
    let ceres: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((CERES_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (CERES_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            ..default()
        },
        mass: Mass(CERES_MASS),
        velocity: Velocity(
            Quat::from_rotation_x(CERES_INCLINATION) * Vec3::new(0.0, CERES_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(CERES_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
        ..default()
    };

    // Jupiter
    let jupiter: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((JUPITER_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (JUPITER_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.76, 0.4, 0.11),
                emissive: (Color::rgb(0.76, 0.4, 0.11) * 2.),
                ..default()
            }),
            ..default()
        },
        mass: Mass(JUPITER_MASS),
        velocity: Velocity(
            Quat::from_rotation_x(JUPITER_INCLINATION) * Vec3::new(0.0, JUPITER_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(JUPITER_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
        ..default()
    };

    // Saturn
    let saturn: BodyBundle = BodyBundle {
        pbr: PbrBundle {
            transform: Transform::from_xyz((SATURN_DISTANCE * SCALE) as f32, 0.0, 0.0),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: (SATURN_RADIUS * SCALE) as f32,
                    subdivisions: 10,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.53, 0.45, 0.28),
                emissive: (Color::rgb(0.53, 0.45, 0.28) * 2.),
                ..default()
            }),
            ..default()
        },
        mass: Mass(SATURN_MASS),
        velocity: Velocity(
            Quat::from_rotation_x(SATURN_INCLINATION) * Vec3::new(0.0, SATURN_VEL, 0.0),
        ),
        coord: Coord(DVec3::new(SATURN_DISTANCE, 0.0, 0.0)),
        circle_size: CircleSize(0.01),
        ..default()
    };

    let sun = commands.spawn((sun, Star)).id();
    let mercury = commands.spawn(mercury).id();
    let venus = commands.spawn(venus).id();
    let earth = commands.spawn(earth).id();
    let moon = commands.spawn(moon).id();
    let mars = commands.spawn(mars).id();
    let ceres = commands.spawn(ceres).id();
    let jupiter = commands.spawn(jupiter).id();
    let saturn = commands.spawn(saturn).id();

    // Label
    let font = asset_server.load("PressStart2P-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 12.0,
        color: Color::ANTIQUE_WHITE,
    };

    let mut label = |entity: Entity, label: &str, shift: f32, threshold: f32| {
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
                    shift,
                    text: label.to_string(),
                    entity,
                    threshold,
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

    label(sun, "Star: Sun", 6.0, (AU * SCALE * 100.0) as f32);
    label(mercury, "Planet: Mercury", 0.02, (AU * SCALE * 10.0) as f32);
    label(venus, "Planet: Venus", 0.05, (AU * SCALE * 10.0) as f32);
    label(earth, "Planet: Earth", 0.05, (AU * SCALE * 10.0) as f32);
    label(moon, "Satellite: Moon", 0.01, (AU * SCALE * 0.1) as f32);
    label(mars, "Planet: Mars", 0.03, (AU * SCALE * 10.0) as f32);
    label(
        ceres,
        "Dwarf Planet: Ceres",
        0.005,
        (AU * SCALE * 50.0) as f32,
    );
    label(jupiter, "Planet: Jupiter", 0.7, (AU * SCALE * 50.0) as f32);
    label(saturn, "Planet: Saturn", 0.6, (AU * SCALE * 50.0) as f32);

    // Camera
    let position =
        Transform::from_xyz(0.0, 0.0, (AU * 3.0 * SCALE) as f32).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform: position,
            ..default()
        },
        Ordinal(0),
        PanOrbitCamera {
            radius: Some((AU * 3.0 * SCALE) as f32),
            pan_smoothness: 0.0,
            ..default()
        },
    ));

    config.line_width = 1.5;

    commands.init_resource::<PanSoft>();
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

        vel1.0 += force1.as_vec3() * TIMESTEP;

        let diff2 = coord1.0 - coord2.0;

        let phi2 = diff2.y.atan2(diff2.x);
        let theta2 = (diff2.z / distance).acos();

        let force2 = (G * m1) / (distance * distance);

        let x2 = force2 * theta2.sin() * phi2.cos();
        let y2 = force2 * theta2.sin() * phi2.sin();
        let z2 = force2 * theta2.cos();

        let force2 = DVec3::new(x2, y2, z2);

        vel2.0 += force2.as_vec3() * TIMESTEP;
    }
}

pub fn update_pansoft(mut pansoft: ResMut<PanSoft>) {
    pansoft.0 -= 0.05;
    pansoft.0 = pansoft.0.max(0.0);
}

pub fn update_position(
    mut bodies: Query<(&Velocity, &mut Transform, &mut Coord, &mut Trajectory)>,
) {
    for (vel, mut transform, mut coord, mut trajectory) in &mut bodies {
        coord.0 += (vel.0 * TIMESTEP).as_dvec3(); //* time.delta_seconds();
        transform.translation = (coord.0 * SCALE).as_vec3();
        trajectory.0.push(coord.0);
    }
}

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    bodies: Query<(&Coord, &Trajectory, &CircleSize)>,
    camera: Query<(&PanOrbitCamera, &Transform)>,
) {
    let (camera, camera_transform) = camera.single();
    for (coord, trajectory, circle) in &bodies {
        let radius = camera.radius.unwrap() * circle.0;
        let normal = camera_transform.rotation * Vec3::Z;

        gizmos.circle(
            (coord.0 * SCALE).as_vec3(),
            normal,
            radius,
            Color::rgb(0.3, 0.0, 0.0),
        );

        for coords in trajectory.0.windows(2) {
            gizmos.line(
                (coords[0] * SCALE).as_vec3(),
                (coords[1] * SCALE).as_vec3(),
                Color::DARK_GRAY,
            );
        }
    }
    gizmos.line(
        Vec3::X * 2.0 * (AU * SCALE) as f32,
        -Vec3::X * 2.0 * (AU * SCALE) as f32,
        Color::MIDNIGHT_BLUE,
    );
    gizmos.line(
        Vec3::Y * 2.0 * (AU * SCALE) as f32,
        -Vec3::Y * 2.0 * (AU * SCALE) as f32,
        Color::SEA_GREEN,
    );
}

pub fn look_at_target(
    mut camera: Query<(&Ordinal, &mut PanOrbitCamera)>,
    bodies: Query<(&Coord, With<Mass>)>,
    pansoft: Res<PanSoft>,
) {
    let (ordinal, mut camera) = camera.single_mut();
    let (coord, _) = bodies.iter().nth(ordinal.0).unwrap();
    let coord = (coord.0 * SCALE).as_vec3();

    camera.target_focus = coord;
    camera.pan_smoothness = pansoft.0;
}

pub fn switch_focus_body(
    keyboard_input: Res<Input<KeyCode>>,
    bodies: Query<(&GlobalTransform, With<Mass>)>,
    mut camera: Query<&mut Ordinal>,
    mut pansoft: ResMut<PanSoft>,
) {
    let mut ordinal = camera.single_mut();
    let count = bodies.iter().count();

    if keyboard_input.just_pressed(KeyCode::Left) {
        pansoft.0 = 0.9;
        ordinal.0 = if ordinal.0 == 0 {
            count - 1
        } else {
            ordinal.0 - 1
        };
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        pansoft.0 = 0.9;
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
        let body_transform = bodies.get(label.entity).unwrap();
        let dist = body_transform.translation().length();
        let world_position =
            body_transform.translation() + Vec3::new(label.shift, -label.shift, 0.0);

        if let Some(viewport_position) =
            camera.world_to_viewport(camera_global_transform, world_position)
        {
            style.top = Val::Px(viewport_position.y);
            style.left = Val::Px(viewport_position.x);

            let dist_camera = (camera_global_transform.translation() - world_position).length();

            text.sections[0].value = if dist_camera <= label.threshold {
                format!("{}\n{:.4} AU", label.text, dist / (AU * SCALE) as f32)
            } else {
                "".to_string()
            };
        }
    }
}
