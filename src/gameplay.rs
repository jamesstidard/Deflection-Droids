use amethyst::{
    SimpleState,
    prelude::*,
    assets::{AssetStorage, Loader},
    renderer::{
        camera::Projection,
        SpriteSheet, SpriteSheetFormat, SpriteRender,
        Texture, ImageFormat, Camera,
    },
    core::transform::Transform,
    window::ScreenDimensions,
};

use crate::utils;
use crate::components;


fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        // Define the view that the camera can see. It makes sense to keep the `near` value as
        // 0.0, as this means it starts seeing anything that is 0 units in front of it. The
        // `far` value is the distance the camera can see facing the origin.
        .with(Camera::from(Projection::orthographic(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            0.0,
            100.0,
        )))
        .with(transform)
        .build();
}


fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/spritesheet.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..5)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}


pub struct Gameplay;

impl SimpleState for Gameplay {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let sprites = load_sprites(world);

        initialise_camera(world);
        world.register::<components::Tile>();
        world.register::<components::Wall>();
        world.register::<components::Droid>();
        utils::board::initialise(world, &sprites);
    }

}
