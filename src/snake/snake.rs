use bevy::prelude::*;

#[derive(Component)]
struct Snake {
    head: Head,
    snakeBody: [bodyPart],
}

#[derive(Component)]
struct Head;

#[derive(Component)]
struct bodyPart;

fn instantiate_snake(mut commands: Commands) {
    commands.spawn(Snake)
}

