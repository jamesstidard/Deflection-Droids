// use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, ReadExpect};
// use amethyst::input::InputHandler;
// use amethyst::renderer::{MouseButton, Camera, ScreenDimensions};
// use amethyst::core::components::Transform;

// use crate::components::Droid;
// use crate::components::Selected;


// pub struct SelectDroidSystem;

// impl<'s> System<'s> for SelectDroidSystem {
//     type SystemData = (
//         Entities<'s>,
//         ReadStorage<'s, Camera>,
//         ReadExpect<'s, ScreenDimensions>,
//         WriteStorage<'s, Selected>,
//         ReadStorage<'s, Droid>,
//         ReadStorage<'s, Transform>,
//         Read<'s, InputHandler<String, String>>,
//     );

//     fn run(&mut self, (entities, cameras, dim, selections, droids, transforms, input): Self::SystemData) {
//         let (mx, my) = input.mouse_position().unwrap_or((0.0, 0.0));
//         let down = input.mouse_button_is_down(MouseButton::Left);

//         let (camera, camera_transform) = (&cameras, &transforms).join().next().unwrap();
//         let (width, height) = { (dim.width(), dim.height()) };

//         let window = Vector3::new(mx as f32, my as f32, 0.);
//         let viewport = Vector4::new(0.0, 0.0, width, height);
//         let vec: Vec<_> = camera_transform
//             .view_matrix()
//             .as_slice()
//             .iter()
//             .map(|v| v.as_f32())
//             .collect();

//         let mouse_pos = unproject(
//             &window,
//             //TODO I'm doing these conversions just to get around a type confusion between
//             //the ameythst and the nalgebra_glm crates. Seems silly.
//             &Matrix4::from_vec(vec),
//             &Matrix4::from_vec(camera.projection().as_matrix().as_slice().into()),
//             viewport,
//         );

//         for (entity, droid, transform) in (&entities, &droids, &transforms).join() {
//             println!("{:?}", mouse_pos);
//             println!("{:?}", transform.translation());
//             println!("{:?}", down);
//         }
//     }
// }

// // Camera used to have a camera.position_from_screen method which went away in the switch to rendy. If you're on 0.10 I think it's still there. Does anyone know if that's gone for good or will be brought back?
// // [7:20 PM] tubotinatub: In the meantime I'm using this which uses nalgebra_glm's unproject function and a little bit of cargo culting because I'm vague on how the math of projections work.

// // impl<'s> System<'s> for MousePositionSystem {
// //     type SystemData = (
// //         WriteStorage<'s, Transform>,
// //         ReadStorage<'s, Camera>,
// //         ReadExpect<'s, InputHandler<StringBindings>>,
// //         ReadExpect<'s, ScreenDimensions>,
// //     );

// // fn run(&mut self, (mut transforms, cameras, input, dim): Self::SystemData) {
// //         let (mx, my) = input.mouse_position().unwrap_or((0.0, 0.0));
// //         let (camera, transform) = (&cameras, &transforms).join().next().unwrap();
// //         let (width, height) = { (dim.width(), dim.height()) };
// //         let win = Vector3::new(mx as f32, my as f32, 0.);
// //         let viewport = Vector4::new(0.0, 0.0, width, height);
// //         let vec:Vec<_> = transform.view_matrix().as_slice().iter().map(|v| v.as_f32()).collect();
// //         let mouse_pos = unproject(
// //             &win,
// //             //TODO I'm doing these conversions just to get around a type confusion between
// //             //the ameythst and the nalgebra_glm crates. Seems silly.
// //             &Matrix4::from_vec(vec),
// //             &Matrix4::from_vec(camera.projection().as_matrix().as_slice().into()),
// //             viewport,
// //   );
// // }

// // [7:21 PM] tubotinatub: That's clipped out of surrounding code so it may not quite work as is, but the logic is there
// // [7:21 PM] tubotinatub: I actually store mouse_pos as a resource and access it from other systems which need to know about the mouse. I'm not sure that's the best solution but it works for my game.
// // [7:23 PM] tubotinatub: To be clear, mouse_pos is the location of the cursor in world space. And I'm assuming your entities are 2D sprites seen through a orthographic camera, if that's not true then what I just posted is garbage.
