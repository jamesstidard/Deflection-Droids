use std::collections::HashSet;

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::components::Parent;

use crate::components::Tile;
use crate::components::Wall;
use crate::components::Selected;
use crate::components::wall::Side;
use crate::components::droid::{Droid};


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MoveDroidSystem;

impl<'s> System<'s> for MoveDroidSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Droid>,
        WriteStorage<'s, Parent>,
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
            walls,
            tiles,
            transforms,
            selections,
            input
        ): Self::SystemData
    )
    {
        let mut moves = Vec::new();

        for (parent_droid, _) in (&parents, &selections).join() {
            let droid_entity = parent_droid.entity;
            let parent_tile = parents.get(droid_entity).unwrap();

            let hori = input.axis_value("move_horizontal");
            let vert = input.axis_value("move_vertical");

            let direction = match (hori, vert) {
                (Some(x), _) if x > 0.0 => Direction::Right,
                (Some(x), _) if x < 0.0 => Direction::Left,
                (_, Some(y)) if y > 0.0 => Direction::Up,
                (_, Some(y)) if y < 0.0 => Direction::Down,
                _ => continue,
            };

            let stopping_wall = match direction {
                Direction::Right => Side::Right,
                Direction::Left => Side::Left,
                Direction::Up => Side::Top,
                Direction::Down => Side::Bottom,
            };

            let tile_transform = transforms.get(parent_tile.entity).unwrap();
            let current_x = tile_transform.translation().x;
            let current_y = tile_transform.translation().y;

            let wall_tile_entities = (&walls, &parents).join()
                .filter(|(wall, _)| wall.side == stopping_wall)
                .map(|(wall, parent)| (wall, parent, transforms.get(parent.entity)))
                .map(|(wall, parent, tf)| (wall, parent, tf.unwrap().translation()))
                .map(|(_, parent, _)| parent.entity)
                .collect::<HashSet<_>>();

            let mut droid_tile_entities = (&droids, &parents).join()
                .map(|(droid, parent)| (droid, parent, transforms.get(parent.entity)))
                .map(|(droid, parent, tf)| (droid, parent, tf.unwrap().translation()))
                .map(|(_, parent, _)| parent.entity)
                .collect::<HashSet<_>>();
            droid_tile_entities.remove(&parent_tile.entity);  // remove self

            let candidate_tiles = match direction {
                Direction::Right => {
                    let mut candidate_tiles = (&*entities, &tiles, &transforms).join()
                        .map(|(ent, tile, tf)| (ent, tile, tf.translation()))
                        .filter(|(_, _, tl)| tl.x >= current_x)
                        .filter(|(_, _, tl)| tl.y == current_y)
                        .collect::<Vec<_>>();

                    candidate_tiles.sort_by(|(_, _, t1), (_, _, t2)|
                        t1.x.partial_cmp(&t2.x).unwrap()
                    );
                    candidate_tiles
                },
                Direction::Left => {
                    let mut candidate_tiles = (&*entities, &tiles, &transforms).join()
                        .map(|(ent, tile, tf)| (ent, tile, tf.translation()))
                        .filter(|(_, _, tl)| tl.x <= current_x)
                        .filter(|(_, _, tl)| tl.y == current_y)
                        .collect::<Vec<_>>();

                    candidate_tiles.sort_by(|(_, _, t1), (_, _, t2)|
                        t1.x.partial_cmp(&t2.x).unwrap()
                    );
                    candidate_tiles.reverse();
                    candidate_tiles
                },
                Direction::Up => {
                    let mut candidate_tiles = (&*entities, &tiles, &transforms).join()
                        .map(|(ent, tile, tf)| (ent, tile, tf.translation()))
                        .filter(|(_, _, tl)| tl.x == current_x)
                        .filter(|(_, _, tl)| tl.y >= current_y)
                        .collect::<Vec<_>>();

                    candidate_tiles.sort_by(|(_, _, t1), (_, _, t2)|
                        t1.y.partial_cmp(&t2.y).unwrap()
                    );
                    candidate_tiles
                },
                Direction::Down => {
                    let mut candidate_tiles = (&*entities, &tiles, &transforms).join()
                        .map(|(ent, tile, tf)| (ent, tile, tf.translation()))
                        .filter(|(_, _, tl)| tl.x == current_x)
                        .filter(|(_, _, tl)| tl.y <= current_y)
                        .collect::<Vec<_>>();

                    candidate_tiles.sort_by(|(_, _, t1), (_, _, t2)|
                        t1.y.partial_cmp(&t2.y).unwrap()
                    );
                    candidate_tiles.reverse();
                    candidate_tiles
                }
            };

            for (tile_entity, _, _) in candidate_tiles {
                if droid_tile_entities.contains(&tile_entity) {
                    break
                }

                moves.push((droid_entity, tile_entity));

                if wall_tile_entities.contains(&tile_entity) {
                    break
                }
            }
        }

        for (droid, tile) in moves {
            parents.insert(droid, Parent{entity: tile});
        }
    }
}
