use anyhow::Result;
use error::AppError;
use image;
use ril::prelude::*;

mod error;

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
    let border_color = Rgba {
        r: 47,
        g: 43,
        b: 34,
        a: 255,
    };
    let inset_color_locked = Rgba {
        r: 117,
        g: 99,
        b: 78,
        a: 255,
    };
    let background_color_locked = Rgba {
        r: 74,
        g: 62,
        b: 50,
        a: 255,
    };
    let mut composited_image = Image::new(256, 256, background_color_locked);

    let border = Rectangle::<Rgba>::from_bounding_box(0, 0, 256, 256)
        .with_border(Border::new(border_color, 4).with_position(BorderPosition::Inset));
    composited_image.draw(&border);

    let inset = Rectangle::<Rgba>::from_bounding_box(4, 4, 252, 252)
        .with_border(Border::new(inset_color_locked, 4).with_position(BorderPosition::Inset));
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

    // text compositing - this is SUPER janky at the moment but it's a first-pass
    let font_bytes =
        include_bytes!("../assets/fonts/runescape-chat-bold-2/runescape-chat-bold-2.otf");
    let font = Font::from_bytes(font_bytes, 12.0).map_err(AppError::RILError)?;
    let text = "Serpentine helm";
    let layout = TextLayout::new()
        .with_position(0, 0)
        .with_wrap(WrapStyle::None)
        .with_segment(
            &TextSegment::new(
                &font,
                text,
                Rgba {
                    r: 255,
                    g: 144,
                    b: 0,
                    a: 255,
                },
            )
            .with_size(20.0),
        );
    // add 1 here because of the shadow
    let (mut text_width, mut text_height) = layout.dimensions();
    text_width += 1;
    text_height += 1;
    let mut shadow_image = Image::new(
        text_width,
        text_height,
        Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    );
    // draw the text then saturate it to pure black
    shadow_image.draw(&layout);
    shadow_image.darken(255);
    let mut text_image = Image::new(
        text_width,
        text_height,
        Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    );
    text_image.draw(&Paste {
        position: (1, 1),
        image: &shadow_image,
        mask: None,
        overlay: Some(OverlayMode::Merge),
    });
    text_image.draw(&layout);

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
