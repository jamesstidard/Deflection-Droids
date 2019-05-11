use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 64.0;


pub struct Tile {
    pub closed_left: bool,
    pub closed_right: bool,
    pub closed_top: bool,
    pub closed_bottom: bool,
}


impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}
