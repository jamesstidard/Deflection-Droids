use amethyst::{
    ecs::prelude::{Component, VecStorage},
};


#[derive(Debug)]
pub struct Selected;


impl Component for Selected {
    type Storage = VecStorage<Self>;
}
