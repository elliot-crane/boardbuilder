use anyhow::Result;
use error::AppError;
use image;
use ril::prelude::*;
use text::TextRenderer;
use tile::{Tile, TileRenderer};

mod error;
mod palette;
mod text;
mod tile;

fn main() -> Result<()> {
    let bytes_serp = include_bytes!("../.cache/Serpentine_helm_detail.png");
    let image_serp = image::load_from_memory(bytes_serp)?.to_rgba8();
    // this is messy, but unfortunately RIL seems to have some issue with decoding palette-indexed transparent PNGs
    // I still wanted to use RIL's compositing and editing API, so I made this hack loading through the image crate
    let ril_serp = Image::from_fn(image_serp.width(), image_serp.height(), |x, y| {
        let p = image_serp.get_pixel(x, y);
        Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        }
    });

    let text_renderer = TextRenderer::default();
    let tile_renderer = TileRenderer::new(Default::default(), &text_renderer);
    let test_tile = Tile {
        number: 14,
        name: "Serpentine helm".to_string(),
        image: ril_serp.clone(),
        unlocked: false,
    };

    let tile_image = tile_renderer.render(&test_tile);
    tile_image
        .save(ImageFormat::Png, ".cache/Serpentine_helm_tile.png")
        .map_err(AppError::RILError)?;
    Ok(())
}
