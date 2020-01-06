extern crate amethyst;
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
};

mod components;
mod systems;
mod gameplay;
mod utils;
use crate::gameplay::Gameplay;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let config = app_root.join("config");
    let display_config = config.join("display.ron");
    let binding_path = config.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(binding_path)?
        )?
        .with(systems::MoveDroidSystem, "move_droid", &[])
        .with(systems::SelectDroidSystem, "select_droid", &[])
        .with(systems::HighlightTileSystem, "highlight_tile", &[])
    ;

    let mut game = Application::new(resources, Gameplay, game_data)?;
    game.run();

    Ok(())
}
