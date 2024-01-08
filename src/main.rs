use bevy::prelude::*;
// use nalgebra::Vector3;
use solar_system_sim::{
    attraction, draw_gizmos, look_at_target, scroll_camera, spawn_bodies, switch_track,
    update_lerp, update_position,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_systems(Startup, spawn_bodies)
        .add_systems(FixedUpdate, (attraction, update_position, update_lerp))
        .add_systems(
            Update,
            (draw_gizmos, scroll_camera, switch_track, look_at_target),
        )
        .run();
}
