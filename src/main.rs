extern crate amethyst;
use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage, ColorMask, ALPHA},
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::InputBundle,
};

mod components;
mod gameplay;
mod utils;
use crate::gameplay::Gameplay;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );
    let render_bundle = RenderBundle::new(pipe, Some(config))
        .with_sprite_sheet_processor();

    // input bundle
    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("./", Gameplay, game_data)?;

    game.run();

    Ok(())
}
