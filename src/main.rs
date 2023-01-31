use anyhow::Result;
use board::{Board, BoardRenderer};
use error::AppError;
use images::ImageLoader;
use ril::prelude::*;
use text::TextRenderer;
use tile::{Tile, TileRenderOptions, TileRenderer};

mod board;
mod builder;
mod error;
mod images;
mod palette;
mod text;
mod tile;

fn main() -> Result<()> {
    // deps
    let image_loader = ImageLoader::new(Default::default())?;
    let text_renderer = TextRenderer::default();
    let tile_renderer = TileRenderer::new(&text_renderer);
    let board_renderer = BoardRenderer::new(&tile_renderer);

    let serp_helm = image_loader
        .load_from_url("https://oldschool.runescape.wiki/images/thumb/Serpentine_helm_detail.png/425px-Serpentine_helm_detail.png")?;
    let serp_tile = Tile {
        number: 14,
        name: "Serpentine helm".to_string(),
        image: serp_helm.clone(),
        unlocked: false,
    };

    let mark_of_grace = image_loader
        .load_from_url("https://oldschool.runescape.wiki/images/thumb/Mark_of_grace_detail.png/487px-Mark_of_grace_detail.png")?;
    let agility_tile = Tile {
        number: 11,
        name: "1M Agility XP".to_string(),
        image: mark_of_grace.clone(),
        unlocked: false,
    };

    let board = Board {
        rows: 5,
        cols: 5,
        content_rect: (20, 20, 120 + 216 * 5, 120 + 216 * 5),
        tile_size: 216,
        tiles: vec![serp_tile, agility_tile],
        tile_render_options: TileRenderOptions::default(),
        image: Image::new(
            140 + 216 * 5,
            140 + 216 * 5,
            Rgba {
                r: 240,
                g: 20,
                b: 200,
                a: 255,
            },
        ),
    };
    let board_image = board_renderer.render(&board);
    board_image
        .save(ImageFormat::Png, "target/board_sample.png")
        .map_err(AppError::RILError)?;

    Ok(())
}
