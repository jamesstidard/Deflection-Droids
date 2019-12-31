use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::components::Parent;

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
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Selected>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (entities, droids, mut parents, walls, transforms, selections, input): Self::SystemData) {
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

            if let Some(tile_transform) = transforms.get(parent_tile.entity) {

                match direction {
                    Direction::Right => {
                        let mut line_tiles = (&walls, &parents)
                            .join()
                            .filter(|(wall, _)| wall.side == Side::Right)
                            .map(|(wall, parent)| (wall, parent, transforms.get(parent.entity)))
                            .filter(|(_, _, transform)| transform.is_some())
                            .map(|(wall, parent, transform)| (wall, parent, transform.unwrap()))
                            .filter(|(_, _, transform)| transform.translation().y == tile_transform.translation().y)
                            .filter(|(_, _, transform)| transform.translation().x >= tile_transform.translation().x)
                            .collect::<Vec<_>>();

                        line_tiles.sort_by(|(_, _, t1), (_, _, t2)| t1.translation().x.partial_cmp(&t2.translation().x).unwrap());

                        let next_tile = line_tiles
                            .iter()
                            .take(1)
                            .map(|(_, parent, _)| parent.entity)
                            .last();

                        if let Some(tile) = next_tile {
                            moves.push((droid_entity, tile));
                        }
                    },
                    Direction::Left => {
                        let mut line_tiles = (&walls, &parents)
                            .join()
                            .filter(|(wall, _)| wall.side == Side::Left)
                            .map(|(wall, parent)| (wall, parent, transforms.get(parent.entity)))
                            .filter(|(_, _, transform)| transform.is_some())
                            .map(|(wall, parent, transform)| (wall, parent, transform.unwrap()))
                            .filter(|(_, _, transform)| transform.translation().y == tile_transform.translation().y)
                            .filter(|(_, _, transform)| transform.translation().x <= tile_transform.translation().x)
                            .collect::<Vec<_>>();

                        line_tiles.sort_by(|(_, _, t1), (_, _, t2)| t1.translation().x.partial_cmp(&t2.translation().x).unwrap());
                        line_tiles.reverse();

                        let next_tile = line_tiles
                            .iter()
                            .take(1)
                            .map(|(_, parent, _)| parent.entity)
                            .last();

                        if let Some(tile) = next_tile {
                            moves.push((droid_entity, tile));
                        }
                    },
                    Direction::Up => {
                        let mut line_tiles = (&walls, &parents)
                            .join()
                            .filter(|(wall, _)| wall.side == Side::Top)
                            .map(|(wall, parent)| (wall, parent, transforms.get(parent.entity)))
                            .filter(|(_, _, transform)| transform.is_some())
                            .map(|(wall, parent, transform)| (wall, parent, transform.unwrap()))
                            .filter(|(_, _, transform)| transform.translation().x == tile_transform.translation().x)
                            .filter(|(_, _, transform)| transform.translation().y >= tile_transform.translation().y)
                            .collect::<Vec<_>>();

                        line_tiles.sort_by(|(_, _, t1), (_, _, t2)| t1.translation().y.partial_cmp(&t2.translation().y).unwrap());

                        let next_tile = line_tiles
                            .iter()
                            .take(1)
                            .map(|(_, parent, _)| parent.entity)
                            .last();

                        if let Some(tile) = next_tile {
                            moves.push((droid_entity, tile));
                        }
                    },
                    Direction::Down => {
                        let mut line_tiles = (&walls, &parents)
                            .join()
                            .filter(|(wall, _)| wall.side == Side::Bottom)
                            .map(|(wall, parent)| (wall, parent, transforms.get(parent.entity)))
                            .filter(|(_, _, transform)| transform.is_some())
                            .map(|(wall, parent, transform)| (wall, parent, transform.unwrap()))
                            .filter(|(_, _, transform)| transform.translation().x == tile_transform.translation().x)
                            .filter(|(_, _, transform)| transform.translation().y <= tile_transform.translation().y)
                            .collect::<Vec<_>>();

                        line_tiles.sort_by(|(_, _, t1), (_, _, t2)| t1.translation().y.partial_cmp(&t2.translation().y).unwrap());
                        line_tiles.reverse();

                        let next_tile = line_tiles
                            .iter()
                            .take(1)
                            .map(|(_, parent, _)| parent.entity)
                            .last();

                        if let Some(tile) = next_tile {
                            moves.push((droid_entity, tile));
                        }
                    }
                };
            }
        }

        for (entity, tile) in moves {
            parents.insert(entity, Parent{entity: tile});
        }
    }
}
