use crate::arena;
use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
// const SPEED: f32 = 5.0;

pub fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut arena::Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
    }
}

pub fn spawn_snake(mut commands: Commands) {
    // Add head
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
        arena::Position { x: 3, y: 3 },
        arena::Size::square(0.8),
    ));

    // Add segment body
    commands.spawn((
        SnakeSegment,
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        },
        arena::Position { x: 3, y: 3 },
        arena::Size::square(0.8),
    ));
}
