use bevy::prelude::*;
pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

pub fn size_scaling(
    mut windows: Query<&mut Window>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = windows.single_mut();
    for (sprite_size, mut transform) in q.iter_mut() {
        let scale_factor = ((window.width() as f32) / (ARENA_WIDTH as f32))
            .min((window.height() as f32) / (ARENA_HEIGHT as f32));

        transform.scale = Vec3::new(
            sprite_size.width * scale_factor,
            sprite_size.height * scale_factor,
            1.0,
        );
    }
}

pub fn position_translation(
    mut windows: Query<&mut Window>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        (pos / bound_game) * bound_window - bound_window / 2.0 + tile_size / 2.0
    }
    let window = windows.single_mut();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
