extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};
mod states;


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
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );
    let render_bundle = RenderBundle::new(pipe, Some(config))
        .with_sprite_sheet_processor();

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?;

    let mut game = Application::new("./", states::Gameplay, game_data)?;

    game.run();

    Ok(())
}
