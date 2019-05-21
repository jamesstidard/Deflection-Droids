use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities};
use amethyst::input::InputHandler;
use amethyst::renderer::MouseButton;
use amethyst::core::components::Transform;

use crate::components::Droid;
use crate::components::Selected;


pub struct SelectDroidSystem;

impl<'s> System<'s> for SelectDroidSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Selected>,
        ReadStorage<'s, Droid>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, selections, droids, transforms, input): Self::SystemData) {
        for (entity, droid, transform) in (&entities, &droids, &transforms).join() {
            let pos = input.mouse_position();
            println!("{:?}", pos);
            println!("{:?}", transform.translation());
            let down = input.mouse_button_is_down(MouseButton::Left);
            println!("{:?} {:?}", pos, down);
        }
    }
}
