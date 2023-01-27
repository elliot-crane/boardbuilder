use anyhow::Result;
use error::AppError;
use images::ImageLoader;
use ril::prelude::*;
use text::TextRenderer;
use tile::{Tile, TileRenderer};

mod error;
mod images;
mod palette;
mod text;
mod tile;

fn main() -> Result<()> {
    // deps
    let image_loader = ImageLoader::new(Default::default())?;
    let text_renderer = TextRenderer::default();
    let tile_renderer = TileRenderer::new(Default::default(), &text_renderer);

    let serp_helm = image_loader
        .load_from_url("https://oldschool.runescape.wiki/images/thumb/Serpentine_helm_detail.png/425px-Serpentine_helm_detail.png")?;
    let test_tile = Tile {
        number: 14,
        name: "Serpentine helm".to_string(),
        image: serp_helm.clone(),
        unlocked: false,
    };

    let tile_image = tile_renderer.render(&test_tile);
    tile_image
        .save(ImageFormat::Png, "target/Serpentine_helm_tile.png")
        .map_err(AppError::RILError)?;
    Ok(())
}
