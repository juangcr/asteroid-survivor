use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.2, 0.5, 1.0), Vec2::new(100.0, 100.0)),
    ));
}
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        let speed = 300.0;

        transform.translation += direction * speed * time.delta_secs();
    }
}

#[derive(Component)]
struct Player;
