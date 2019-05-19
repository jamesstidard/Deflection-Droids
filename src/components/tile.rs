use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 64.0;


#[derive(Debug)]
pub struct Tile {

}


impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}
