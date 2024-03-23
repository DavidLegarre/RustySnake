use bevy::{ prelude::* };

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        SnakeHead,
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<(&SnakeHead, &mut Transform)>
) {
    const SPEED: f32 = 2.0;
    let mut direction = Vec3::ZERO;
    for (_head, mut transform) in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += SPEED;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += SPEED;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize();
        }

        // direction = Vec3::ZERO;
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
