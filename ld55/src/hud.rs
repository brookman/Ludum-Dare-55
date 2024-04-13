use bevy::prelude::*;
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Hud;

#[allow(clippy::needless_pass_by_value)]
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::GRAY,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("Welcome to Ludum Dare 55", text_style).with_style(
                    Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                ),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        });
}

#[allow(clippy::needless_pass_by_value)]
fn update(mut hud_visibility: Query<&mut Visibility, With<Hud>>) {
    let show = true;
    *hud_visibility.single_mut() = if show {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
