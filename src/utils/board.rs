use amethyst::{
    ecs::prelude::*,
    core::transform::Transform,
    renderer::{SpriteSheetHandle, SpriteRender},
};

use crate::components::Tile;
use crate::components::tile::{TILE_HEIGHT, TILE_WIDTH};

pub const X_TILES_COUNT: i32 = 16;
pub const Y_TILES_COUNT: i32 = 16;
pub const BOARD_WIDTH: f32 = TILE_WIDTH * (X_TILES_COUNT as f32);
pub const BOARD_HEIGHT: f32 = TILE_HEIGHT * (Y_TILES_COUNT as f32);


pub fn initialise(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let tile_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    for x_tile in 0..X_TILES_COUNT {
        for y_tile in 0..Y_TILES_COUNT {

            let mut local_transform = Transform::default();
            let x = (x_tile as f32) * TILE_WIDTH + TILE_WIDTH * 0.5;
            let y = (y_tile as f32) * TILE_HEIGHT + TILE_HEIGHT * 0.5;
            local_transform.set_xyz(x, y, 0.0);

            println!("{} {}", y_tile, (y_tile as f32));

            world
                .create_entity()
                .with(Tile {
                    closed_right: true,
                    closed_left: false,
                    closed_top: false,
                    closed_bottom: false,
                })
                .with(tile_render.clone())
                .with(local_transform)
                .build();
        }
    }
}
