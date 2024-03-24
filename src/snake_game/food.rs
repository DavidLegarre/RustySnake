use crate::arena;
use bevy::prelude::*;
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
struct Food;

pub fn food_spawner(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
        Food,
        arena::Position {
            x: (random::<f32>() * arena::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * arena::ARENA_HEIGHT as f32) as i32,
        },
        arena::Size::square(1.0),
    ));
}
