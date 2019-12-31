use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, ReadExpect};
use amethyst::assets::{AssetStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{
    camera::{Camera, ActiveCamera},
    sprite::{SpriteRender, SpriteSheet},
};
use amethyst::core::{
    components::Transform,
    geometry::Plane,
    math::{Point2, Vector2},
};
use amethyst::window::ScreenDimensions;
use amethyst::winit::{MouseButton};

use crate::components::Droid;
use crate::components::Selected;


pub struct SelectDroidSystem;

impl<'s> System<'s> for SelectDroidSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Selected>,
        ReadStorage<'s, Droid>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<SpriteSheet>>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (entities, cameras, dim, selections, droids, sprites, transforms, sprite_sheets, active_camera, input): Self::SystemData) {
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            let down = input.mouse_button_is_down(MouseButton::Left);

            if !down {
                return
            }

            let mut camera_join = (&cameras, &transforms).join();

            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_x, mouse_y),
                    Vector2::new(dim.width(), dim.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                for (droid, sprite, transform) in (&droids, &sprites, &transforms).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - sprite.width,
                            transform.translation().x + sprite.width,
                            transform.translation().y - sprite.height,
                            transform.translation().y + sprite.height,
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        println!("{:?}", droid);
                    }
                }
            }
        }
    }
}
