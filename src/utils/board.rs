use amethyst::{
    ecs::prelude::*,
    core::transform::{Transform, Parent},
    renderer::{SpriteRender, Transparent},
    window::ScreenDimensions,
};

use crate::components::Tile;
use crate::components::tile::{TILE_HEIGHT, TILE_WIDTH};
use crate::components::Wall;
use crate::components::wall::Side;
use crate::components::Droid;

pub const X_TILES_COUNT: i32 = 16;
pub const Y_TILES_COUNT: i32 = 16;
pub const BOARD_WIDTH: f32 = TILE_WIDTH * (X_TILES_COUNT as f32);
pub const BOARD_HEIGHT: f32 = TILE_HEIGHT * (Y_TILES_COUNT as f32);

const WALLS: [[f32; 2]; 50] = [
    [6.0, 0.5],
    [13.0, 0.5],
    [3.5, 1.0],
    [4.0, 1.5],
    [11.0, 1.5],
    [0.5, 2.0],
    [10.5, 2.0],
    [8.0, 2.5],
    [8.5, 3.0],
    [7.0, 3.5],
    [6.5, 4.0],
    [15.5, 4.0],
    [1.0, 4.5],
    [12.0, 4.5],
    [1.5, 5.0],
    [5.5, 5.0],
    [9.5, 5.0],
    [12.5, 5.0],
    [5.0, 5.5],
    [10.0, 5.5],
    [14.5, 6.0],
    [14.0, 6.5],
    [7.5, 7.0],
    [8.5, 7.0],
    [7.0, 7.5],
    [9.0, 7.5],
    [7.0, 8.5],
    [9.0, 8.5],
    [7.5, 9.0],
    [8.5, 9.0],
    [0.5, 9.0],
    [5.0, 9.5],
    [4.5, 10.0],
    [15.5, 10.0],
    [12.0, 10.5],
    [1.5, 11.0],
    [10.5, 11.0],
    [12.5, 11.0],
    [1.0, 11.5],
    [10.0, 11.5],
    [6.5, 12.0],
    [7.0, 12.5],
    [14.0, 12.5],
    [11.5, 13.0],
    [13.5, 13.0],
    [12.0, 13.5],
    [3.0, 14.5],
    [3.5, 15.0],
    [2.0, 15.5],
    [12.0, 15.5],
];
const DROIDS: [[i32; 2]; 1] = [
    [0, 0],
];

pub fn initialise(world: &mut World, sprites: &[SpriteRender]) {
    let tile_render = &sprites[0];
    let wall_render = &sprites[3];
    let droid_render = &sprites[1];

    for x_tile in 0..X_TILES_COUNT {
        for y_tile in 0..Y_TILES_COUNT {

            let mut local_transform = Transform::default();
            let x = (x_tile as f32) * TILE_WIDTH + TILE_WIDTH * 0.5;
            let y = (y_tile as f32) * TILE_HEIGHT + TILE_HEIGHT * 0.5;
            local_transform.set_translation_xyz(x, y, -10.0);

            let tile = world
                .create_entity()
                .with(Tile{})
                .with(tile_render.clone())
                .with(local_transform.clone())
                .build();

            // left wall
            let left_edge = x_tile == 0;
            let left_wall = WALLS
                .iter()
                .find(|&&w| w == [(x_tile as f32)+0.0, (y_tile as f32)+0.5])
                .is_some();
            if left_edge || left_wall {
                let mut wall_transform = Transform::default();
                wall_transform.prepend_rotation_z_axis(0f32.to_radians());
                wall_transform.prepend_translation_z(1.0);
                world
                    .create_entity()
                    .with(Wall{side: Side::Left})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .with(Transparent)
                    .build();
            }

            // top wall
            let top_edge = y_tile == Y_TILES_COUNT-1;
            let top_wall = WALLS
                .iter()
                .find(|&&w| w == [(x_tile as f32)+0.5, (y_tile as f32)+1.0])
                .is_some();
            if top_edge || top_wall {
                let mut wall_transform = Transform::default();
                wall_transform.prepend_rotation_z_axis(90f32.to_radians());
                wall_transform.prepend_translation_z(1.0);
                world
                    .create_entity()
                    .with(Wall{side: Side::Top})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .with(Transparent)
                    .build();
            }

            // right wall
            let right_edge = x_tile == X_TILES_COUNT-1;
            let right_wall = WALLS
                .iter()
                .find(|&&w| w == [(x_tile as f32)+1.0, (y_tile as f32)+0.5])
                .is_some();
            if right_edge || right_wall {
                let mut wall_transform = Transform::default();
                wall_transform.prepend_rotation_z_axis(180f32.to_radians());
                wall_transform.prepend_translation_z(1.0);
                world
                    .create_entity()
                    .with(Wall{side: Side::Right})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .with(Transparent)
                    .build();
            }

            // bottom wall
            let bottom_edge = y_tile == 0;
            let bottom_wall = WALLS
                .iter()
                .find(|&&w| w == [(x_tile as f32)+0.5, (y_tile as f32)+0.0])
                .is_some();
            if bottom_edge || bottom_wall {
                let mut wall_transform = Transform::default();
                wall_transform.prepend_rotation_z_axis(270f32.to_radians());
                wall_transform.prepend_translation_z(1.0);
                world
                    .create_entity()
                    .with(Wall{side: Side::Bottom})
                    .with(wall_transform)
                    .with(Parent{entity: tile})
                    .with(wall_render.clone())
                    .with(Transparent)
                    .build();
            }

            // Droid
            if DROIDS.iter().find(|&&w| w == [x_tile, y_tile]).is_some() {
                let mut droid_transform = Transform::default();
                droid_transform.prepend_translation_z(1.0);
                world
                    .create_entity()
                    .with(Droid{})
                    .with(droid_transform)
                    .with(Parent{entity: tile})
                    .with(droid_render.clone())
                    .with(Transparent)
                    .build();
            }
        }
    }
}
