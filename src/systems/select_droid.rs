use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, ReadExpect};
use amethyst::assets::{AssetStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{
    camera::{Camera, ActiveCamera},
    sprite::{SpriteRender, SpriteSheet},
    Transparent,
};
use amethyst::core::{
    components::{Transform, Parent},
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
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transparent>,
        Read<'s, AssetStorage<SpriteSheet>>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (entities, cameras, dim, mut selections, droids, mut sprites, mut transforms, mut parents, mut transparents, sprite_sheets, active_camera, input): Self::SystemData) {
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            let down = input.mouse_button_is_down(MouseButton::Left);

            if !down {
                return
            }

            // deselect previous
            for (e, _) in (&*entities, &selections).join() {
                println!("deleted: {:?}", e);
                entities.delete(e);
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

                for (entity, _, sprite, transform) in (&entities, &droids, &sprites, &transforms).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    // TODO: need transform.translation in world coord system
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - (sprite.width * 0.5),
                            transform.translation().x + (sprite.width * 0.5),
                            transform.translation().y - (sprite.height * 0.5),
                            transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        // let selection_sprite = SpriteRender{
                        //     sprite_sheet: sprite_sheet.clone(),
                        //     sprite_number: 4,
                        // };
                        let mut selection_transform = Transform::default();
                        selection_transform.prepend_translation_z(1.0);
                        let s = entities.build_entity()
                            .with(Selected{}, &mut selections)
                            .with(selection_transform, &mut transforms)
                            .with(Parent{entity: entity}, &mut parents)
                            // .with(selection_sprite.clone(), &mut sprites)
                            .with(Transparent, &mut transparents)
                            .build();

                        println!("built: {:?}", s);

                        // exit after 1st select
                        return
                    }
                }
            }
        }
    }
}
