use bevy::{
    app::{App, Update},
    ecs::system::{ResMut, Resource},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>();
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, update);
    }
}

#[derive(Resource)]
pub struct State {
    pub paused: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { paused: false }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update(mut contexts: EguiContexts, mut ui_state: ResMut<State>) {
    let egui_context = contexts.ctx_mut();

    let mut window = egui::Window::new("Debug UI");
    window = window.resize(|_| {
        bevy_egui::egui::Resize::default()
            .with_stroke(true)
            .min_size([96.0, 32.0])
            .default_size([340.0, 840.0])
    });
    window = window.scroll2([false, true]);

    window.show(egui_context, |ui| {
        ui.vertical(|ui| {
            ui.add_space(10f32);
            ui.label(format!("Paused: {}", ui_state.paused));
            if ui.button("Toggle pause").clicked() {
                ui_state.paused = !ui_state.paused;
            }
        });
    });
}
