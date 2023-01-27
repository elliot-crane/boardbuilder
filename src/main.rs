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
    let serp_tile = Tile {
        number: 14,
        name: "Serpentine helm".to_string(),
        image: serp_helm.clone(),
        unlocked: false,
    };

    tile_renderer
        .render(&serp_tile)
        .save(ImageFormat::Png, "target/Serpentine_helm_tile.png")
        .map_err(AppError::RILError)?;

    let mark_of_grace = image_loader
        .load_from_url("https://oldschool.runescape.wiki/images/thumb/Mark_of_grace_detail.png/487px-Mark_of_grace_detail.png")?;
    let agility_tile = Tile {
        number: 11,
        name: "1M Agility XP".to_string(),
        image: mark_of_grace.clone(),
        unlocked: false,
    };

    tile_renderer
        .render(&agility_tile)
        .save(ImageFormat::Png, "target/Agility_xp_tile.png")
        .map_err(AppError::RILError)?;

    Ok(())
}
