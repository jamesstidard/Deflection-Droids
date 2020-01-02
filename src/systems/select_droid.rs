use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, ReadExpect};
use amethyst::assets::{AssetStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{
    camera::{Camera, ActiveCamera},
    sprite::{SpriteRender, SpriteSheet},
    Transparent,
    resources::Tint,
    palette::Srgba,
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
        ReadStorage<'s, Droid>,
        Read<'s, AssetStorage<SpriteSheet>>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Selected>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transparent>,
        WriteStorage<'s, Tint>,
    );

    fn run(
        &mut self,
        (
            entities,
            cameras,
            dim,
            droids,
            sprite_sheets,
            active_camera,
            input,
            mut selections,
            mut sprites,
            mut transforms,
            mut parents,
            mut transparents,
            mut tints,
        ): Self::SystemData
    ) {
        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            let down = input.mouse_button_is_down(MouseButton::Left);

            if !down {
                return
            }

            // deselect previous
            for (e, _) in (&*entities, &selections).join() {
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

                for (entity, _, sprite_render, parent) in (&entities, &droids, &sprites, &parents).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite_render.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite_render.sprite_number];
                    // TODO: need transform.translation in world coord system
                    let transform = transforms.get(parent.entity).unwrap();
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
                        let selection_sprite = SpriteRender{
                            sprite_sheet: sprite_render.sprite_sheet.clone(),
                            sprite_number: 4,
                        };
                        let mut selection_transform = Transform::default();
                        selection_transform.prepend_translation_z(1.0);
                        let selection_tint = Tint(Srgba::new(0.0, 0.0, 1.0, 1.0));
                        let s = entities.build_entity()
                            .with(Selected{}, &mut selections)
                            .with(selection_transform, &mut transforms)
                            .with(Parent{entity: entity}, &mut parents)
                            .with(selection_sprite.clone(), &mut sprites)
                            .with(selection_tint, &mut tints)
                            .with(Transparent, &mut transparents)
                            .build();

                        // exit after 1st select
                        return
                    }
                }
            }
        }
    }
}
