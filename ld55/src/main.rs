use bevy::{prelude::*, window::WindowResolution};

mod hud;
mod ui;
mod camera;

static APP_NAME: &str = "Ludum Dare 55";

fn main() {
    let mut app = App::new();

    // Default plugins -------------------------------------

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.to_string(),
            // present_mode: PresentMode::AutoNoVsync, // unlimited FPS
            resolution: WindowResolution::new(1920.0, 1080.0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let log_plugin = bevy::log::LogPlugin {
        level: bevy::log::Level::INFO,
        // filter: "wgpu=warn,bevy_ecs=info".to_string(),
        ..default()
    };
    let _asset_plugin = AssetPlugin {
        watch_for_changes_override: Some(true),
        ..Default::default()
    };
    let default_plugins = DefaultPlugins.set(window_plugin).set(log_plugin); //.set(asset_plugin)

    // -----------------------------------------------------

    app.add_plugins((
        default_plugins,
        // FrameTimeDiagnosticsPlugin::default(),
        // LogDiagnosticsPlugin::default(), // log FPS
        ui::Plugin,
        camera::Plugin,
        hud::Plugin,
    ));

    // workaround to fix scroll issues
    // see: https://bevyengine.org/news/bevy-0-13/#events-live-longer
    // app.world.remove_resource::<EventUpdateSignal>();

    app.run();
}
