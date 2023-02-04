use std::{env, fs::File};

use anyhow::Result;
use board::BoardRenderer;
use builder::BoardBuilder;
use error::AppError;
use images::ImageLoader;
use ril::prelude::*;
use text::TextRenderer;
use tile::TileRenderer;

mod board;
mod builder;
mod error;
mod images;
mod palette;
mod text;
mod tile;

fn main() -> Result<()> {
    // TODO: arg parser
    let args = env::args();
    assert_eq!(args.len(), 3, "wrong number of arguments");
    let args = args.into_iter().skip(1).collect::<Vec<_>>();
    let [input_path, output_path]: &[String; 2] = args[..].try_into().unwrap();

    // deps
    let image_loader = ImageLoader::new(Default::default())?;
    let text_renderer = TextRenderer::default();
    let tile_renderer = TileRenderer::new(&text_renderer);
    let board_renderer = BoardRenderer::new(&tile_renderer);

    // loading
    let board_builder: BoardBuilder = serde_yaml::from_reader(File::open(input_path)?)?;
    let board = board_builder.build(&image_loader)?;

    let board_image = board_renderer.render(&board);
    board_image
        .save(ImageFormat::Png, output_path)
        .map_err(AppError::RILError)?;

    Ok(())
}
