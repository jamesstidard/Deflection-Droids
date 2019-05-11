use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};


pub struct Wall {
}


impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
