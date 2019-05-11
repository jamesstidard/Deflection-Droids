use amethyst::{
    ecs::prelude::*,
    core::transform::{Transform, Parent},
    renderer::{SpriteSheetHandle, SpriteRender, Flipped},
};

use crate::components::Tile;
use crate::components::tile::{TILE_HEIGHT, TILE_WIDTH};
use crate::components::Wall;

pub const X_TILES_COUNT: i32 = 16;
pub const Y_TILES_COUNT: i32 = 16;
pub const BOARD_WIDTH: f32 = TILE_WIDTH * (X_TILES_COUNT as f32);
pub const BOARD_HEIGHT: f32 = TILE_HEIGHT * (Y_TILES_COUNT as f32);


pub fn initialise(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let tile_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let wall_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 3,
    };

    for x_tile in 0..X_TILES_COUNT {
        for y_tile in 0..Y_TILES_COUNT {

            let mut local_transform = Transform::default();
            let x = (x_tile as f32) * TILE_WIDTH + TILE_WIDTH * 0.5;
            let y = (y_tile as f32) * TILE_HEIGHT + TILE_HEIGHT * 0.5;
            local_transform.set_xyz(x, y, 0.0);

            let tile = world
                .create_entity()
                .with(Tile{})
                .with(tile_render.clone())
                .with(local_transform.clone())
                .build();

            // left wall
            if x_tile == 0 {
                let mut wall_transform = Transform::default();
                wall_transform.roll_local(0f32.to_radians());
                world
                    .create_entity()
                    .with(Wall{})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .build();
            }

            // top wall
            if y_tile == Y_TILES_COUNT-1 {
                let mut wall_transform = Transform::default();
                wall_transform.roll_local(90f32.to_radians());
                world
                    .create_entity()
                    .with(Wall{})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .build();
            }

            // right wall
            if x_tile == X_TILES_COUNT-1 {
                let mut wall_transform = Transform::default();
                wall_transform.roll_local(180f32.to_radians());
                world
                    .create_entity()
                    .with(Wall{})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .build();
            }

            // bottom wall
            if y_tile == 0 {
                let mut wall_transform = Transform::default();
                wall_transform.roll_local(270f32.to_radians());
                world
                    .create_entity()
                    .with(Wall{})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .build();
            }
        }
    }
}
