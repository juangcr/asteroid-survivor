use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.2, 0.5, 1.0), Vec2::new(100.0, 100.0)),
    ));
}

#[derive(Component)]
struct Player;
