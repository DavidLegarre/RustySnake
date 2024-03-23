mod arena;
mod snake;

use bevy::{ prelude::* };

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    snake::spawn_snake(commands);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake!".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                snake::snake_movement,
                arena::size_scaling,
                arena::position_translation,
            ).chain()
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
