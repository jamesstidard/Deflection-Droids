use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


#[derive(PartialEq, Eq, Debug)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}


#[derive(Debug)]
pub struct Wall {
    pub side: Side,
}


impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
