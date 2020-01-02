use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


#[derive(Debug)]
pub struct Token {
}


impl Component for Token {
    type Storage = DenseVecStorage<Self>;
}
