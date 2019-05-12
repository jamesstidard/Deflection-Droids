use amethyst::{
    SimpleState,
    prelude::*,
    assets::{AssetStorage, Loader},
    renderer::{
        SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Texture, TextureMetadata, PngFormat, Camera, Projection,
    },
    core::transform::Transform,
};

use crate::utils;
use crate::components;


fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            utils::board::BOARD_WIDTH,
            0.0,
            utils::board::BOARD_HEIGHT,
        )))
        .with(transform)
        .build();
}


fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}


pub struct Gameplay;

impl SimpleState for Gameplay {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let spritesheet = load_spritesheet(world);
        initialise_camera(world);
        world.register::<components::Tile>();
        world.register::<components::Wall>();
        world.register::<components::Droid>();
        utils::board::initialise(world, spritesheet);
    }

}
