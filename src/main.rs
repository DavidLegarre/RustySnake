mod snake_game {
    pub mod food;
    pub mod snake;
}
mod arena;

use snake_game::food;
use snake_game::snake;

use bevy::prelude::*;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    snake::spawn_snake(commands);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                snake::snake_movement,
                arena::size_scaling,
                arena::position_translation,
            )
                .chain(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
