use anyhow::Result;
use error::AppError;
use image;
use palette::{DEFAULT_BACKGROUND_LOCKED_COLOR, DEFAULT_BORDER_COLOR, DEFAULT_INSET_COLOR, ORANGE};
use ril::prelude::*;
use text::{TextRenderOptions, TextRenderer};

mod error;
mod palette;
mod text;

fn main() -> Result<()> {
    let bytes_serp = include_bytes!("../.cache/Serpentine_helm_detail.png");
    let image_serp = image::load_from_memory(bytes_serp)?.to_rgba8();
    // this is messy, but unfortunately RIL seems to have some issue with decoding palette-indexed transparent PNGs
    // I still wanted to use RIL's compositing and editing API, so I made this hack loading through the image crate
    let mut ril_serp = Image::from_fn(image_serp.width(), image_serp.height(), |x, y| {
        let p = image_serp.get_pixel(x, y);
        Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        }
    });
    let target_square = 150u32;
    let new_width;
    let new_height;
    if ril_serp.width() > ril_serp.height() {
        let scale_factor = ril_serp.width() as f32 / target_square as f32;
        new_width = target_square;
        new_height = (ril_serp.height() as f32 / scale_factor).floor() as u32;
    } else {
        let scale_factor = ril_serp.height() as f32 / target_square as f32;
        new_width = (ril_serp.width() as f32 / scale_factor).floor() as u32;
        new_height = target_square;
    }
    ril_serp.resize(new_width, new_height, ResizeAlgorithm::Bicubic);

    // compositing over a background image
    let mut composited_image = Image::new(256, 256, DEFAULT_BACKGROUND_LOCKED_COLOR);

    let border = Rectangle::<Rgba>::from_bounding_box(0, 0, 256, 256)
        .with_border(Border::new(DEFAULT_BORDER_COLOR, 4).with_position(BorderPosition::Inset));
    composited_image.draw(&border);

    let inset = Rectangle::<Rgba>::from_bounding_box(4, 4, 252, 252)
        .with_border(Border::new(DEFAULT_INSET_COLOR, 4).with_position(BorderPosition::Inset));
    composited_image.draw(&inset);

    // composite the item image
    let delta_x = composited_image.width() - ril_serp.width();
    let delta_y = composited_image.height() - ril_serp.height();
    let px = if delta_x % 2 == 0 {
        delta_x / 2
    } else {
        1 + delta_x / 2
    };
    let py = if delta_y % 2 == 0 {
        delta_y / 2
    } else {
        1 + delta_y / 2
    };
    composited_image.draw(&Paste {
        position: (px, py),
        image: &ril_serp,
        mask: None,
        overlay: Some(OverlayMode::Merge),
    });

    let text_renderer = TextRenderer::default();
    let text_image = text_renderer.render(
        "Serpentine helm",
        &TextRenderOptions {
            color: ORANGE,
            ..Default::default()
        },
    );
    text_image
        .save(ImageFormat::Png, ".cache/Serpentine_helm_text.png")
        .map_err(AppError::RILError)?;

    // now to add the text to the composited image
    let delta_x = composited_image.width() - text_image.width();
    let px = if delta_x % 2 == 0 {
        delta_x / 2
    } else {
        1 + delta_x / 2
    };
    // border is 8px, so move up an additional 4 for a tiny bit of breathing room
    let py = composited_image.height() - 12 - text_image.height();

    composited_image.draw(&Paste {
        position: (px, py),
        image: &text_image,
        mask: None,
        overlay: Some(OverlayMode::Merge),
    });

    composited_image
        .save(ImageFormat::Png, ".cache/Serpentine_helm_composited.png")
        .map_err(AppError::RILError)?;
    Ok(())
}
