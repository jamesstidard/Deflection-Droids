use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


pub struct Droid {
}


impl Component for Droid {
    type Storage = DenseVecStorage<Self>;
}
