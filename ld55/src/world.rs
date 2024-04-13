use bevy::prelude::*;
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Ground;

#[allow(clippy::needless_pass_by_value)]
fn setup(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            ..default()
        },
        Ground,
    ));
}

fn update() {}
