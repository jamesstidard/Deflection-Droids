use std::collections::HashSet;

use amethyst::core::{Transform};
use amethyst::renderer::{resources::Tint, palette::Srgba};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, Entity};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::components::Parent;

use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Left};

use crate::components::Tile;
use crate::components::Wall;
use crate::components::Selected;
use crate::components::wall::Side;
use crate::components::droid::{Droid};


pub struct HighlightTileSystem;

impl<'s> System<'s> for HighlightTileSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Droid>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Wall>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Selected>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            entities,
            droids,
            mut parents,
            mut tints,
            walls,
            tiles,
            transforms,
            selections,
            input
        ): Self::SystemData
    )
    {
        let mut highlighted = Vec::new();

        for (parent_droid, _) in (&parents, &selections).join() {
            let droid_entity = parent_droid.entity;
            let parent_tile = parents.get(droid_entity).unwrap();

            let tile_transform = transforms.get(parent_tile.entity).unwrap();
            let current_x = tile_transform.translation().x;
            let current_y = tile_transform.translation().y;

            // set of all tile entities occupied by a robot (not including starting tile)
            let occupied_tile_entities = (&droids, &parents).join()
                .map(|(_, parent)| parent.entity)
                .filter(|entity| *entity != parent_tile.entity)
                .collect::<HashSet<_>>();

            // HashMap keyed by wall side with a value of tile entities
            let wall_tile_entities = (&walls, &parents).join()
                .map(|(wall, parent)| (&wall.side, parent.entity))
                .into_group_map();

            highlighted.push(parent_tile.entity);

            let left_candidates = (&*entities, &tiles, &transforms).join()
                .map(|(entity, _, tf)| (entity, tf.translation()))
                .filter(|(_, tl)| tl.x < current_x)
                .filter(|(_, tl)| tl.y == current_y)
                .sorted_by(|(_, tl1), (_, tl2)| tl2.x.partial_cmp(&tl1.x).unwrap())
                .map(|(entity, _)| entity)
                .take_while(|tile| !wall_tile_entities[&Side::Right].contains(&tile) && !occupied_tile_entities.contains(&tile))
                .collect::<HashSet<_>>();

            highlighted.extend(&left_candidates);

            let right_candidates = (&*entities, &tiles, &transforms).join()
                .map(|(entity, _, tf)| (entity, tf.translation()))
                .filter(|(_, tl)| tl.x > current_x)
                .filter(|(_, tl)| tl.y == current_y)
                .sorted_by(|(_, tl1), (_, tl2)| tl1.x.partial_cmp(&tl2.x).unwrap())
                .map(|(entity, _)| entity)
                .take_while(|tile| !wall_tile_entities[&Side::Left].contains(&tile) && !occupied_tile_entities.contains(&tile))
                .collect::<HashSet<_>>();

            highlighted.extend(&right_candidates);

            let up_candidates = (&*entities, &tiles, &transforms).join()
                .map(|(entity, _, tf)| (entity, tf.translation()))
                .filter(|(_, tl)| tl.x == current_x)
                .filter(|(_, tl)| tl.y > current_y)
                .sorted_by(|(_, tl1), (_, tl2)| tl1.y.partial_cmp(&tl2.y).unwrap())
                .map(|(entity, _)| entity)
                .take_while(|tile| !wall_tile_entities[&Side::Bottom].contains(&tile) && !occupied_tile_entities.contains(&tile))
                .collect::<HashSet<_>>();

            highlighted.extend(&up_candidates);

            let down_candidates = (&*entities, &tiles, &transforms).join()
                .map(|(entity, _, tf)| (entity, tf.translation()))
                .filter(|(_, tl)| tl.x == current_x)
                .filter(|(_, tl)| tl.y < current_y)
                .sorted_by(|(_, tl1), (_, tl2)| tl2.y.partial_cmp(&tl1.y).unwrap())
                .map(|(entity, _)| entity)
                .take_while(|tile| !wall_tile_entities[&Side::Top].contains(&tile) && !occupied_tile_entities.contains(&tile))
                .collect::<HashSet<_>>();

            highlighted.extend(&down_candidates);
        }

        for (entity, _) in (&*entities, &tiles).join() {
            if !highlighted.contains(&entity) && !highlighted.is_empty() {
                let tile_tint = Tint(Srgba::new(0.2, 0.2, 0.2, 1.0));
                tints.insert(entity, tile_tint);
            }
            else {
                tints.remove(entity);
            }
        }
    }
}
