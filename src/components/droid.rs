use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


pub enum Direction {
    Horizontal,
    Vertical,
}


#[derive(Debug)]
pub struct Droid {
}


impl Component for Droid {
    type Storage = DenseVecStorage<Self>;
}
