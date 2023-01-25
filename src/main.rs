use anyhow::Result;
use image;
use ril::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("RIL error: {0}")]
    RILError(ril::Error),
}

fn main() -> Result<()> {
    let bytes_serp = include_bytes!("../.cache/Serpentine_helm_detail.png");
    let image_serp = image::load_from_memory(bytes_serp)?.to_rgba8();
    let mut ril_serp = Image::from_fn(image_serp.width(), image_serp.height(), |x, y| {
        let p = image_serp.get_pixel(x, y);
        Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        }
    });
    let target_square = 256u32;
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

    ril_serp
        .save(ImageFormat::Png, ".cache/Serpentine_helm_resized.png")
        .map_err(AppError::RILError)?;
    Ok(())
}
