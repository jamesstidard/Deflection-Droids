use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};


pub struct Droid {
}


impl Component for Droid {
    type Storage = DenseVecStorage<Self>;
}
