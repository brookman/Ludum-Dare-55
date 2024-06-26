use bevy::prelude::*;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin);
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        RtsCamera::default(),
        RtsCameraControls::default(), // Optional
    ));

    // let translation = Vec3::new(0.0, 0.0, -0.25);

    // let focus = Vec3::new(0.0, 0.0, 0.0);
    // let radius = Some((translation - focus).length());

    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_translation(translation).looking_at(focus, Vec3::Y),
    //         projection: Projection::Perspective(PerspectiveProjection {
    //             near: 0.001,
    //             far: 1.0,
    //             ..default()
    //         }),
    //         ..default()
    //     },
    //     PanOrbitCamera {
    //         focus,
    //         radius,
    //         button_pan: MouseButton::Right,
    //         button_orbit: MouseButton::Middle,
    //         orbit_smoothness: 0.0,
    //         pan_smoothness: 0.0,
    //         zoom_smoothness: 0.0,
    //         ..default()
    //     },
    // ));
}
