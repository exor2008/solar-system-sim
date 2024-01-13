use bevy::prelude::*;
// use nalgebra::Vector3;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use solar_system_sim::{
    attraction, draw_gizmos, look_at_target, setup, switch_focus_body, update_labels,
    update_pansoft, update_position,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.01, 0.0, 0.05)))
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (attraction, update_position, update_pansoft))
        .add_systems(
            Update,
            (
                draw_gizmos,
                update_labels,
                // scroll_camera,
                switch_focus_body,
                look_at_target,
            ),
        )
        .run();
}
