use ril::{Font, Image, OverlayMode, Paste, Rgba, TextLayout, TextSegment, WrapStyle};

use crate::error::AppError;

const DEFAULT_FONT_BYTES: &[u8] =
    include_bytes!("../assets/fonts/runescape-chat-bold-2/runescape-chat-bold-2.otf");
const BLACK: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
const TRANSPARENT: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub struct TextRenderer {
    font: Font,
}

impl Default for TextRenderer {
    fn default() -> Self {
        // SAFETY: this is safe to unwrap since the tests below ensure
        // that this works on the target platform
        Self::from_font_bytes(DEFAULT_FONT_BYTES, 20.0).unwrap()
    }
}

impl TextRenderer {
    pub fn from_font(font: Font) -> Self {
        Self { font }
    }

    pub fn from_font_bytes(font_bytes: &[u8], optimal_size: f32) -> Result<Self, AppError> {
        let font = Font::from_bytes(font_bytes, optimal_size).map_err(AppError::RILError)?;
        Ok(Self::from_font(font))
    }

    pub fn render(&self, text: impl AsRef<str>, color: Rgba) -> Image<Rgba> {
        // render text as pure black first
        let layout = TextLayout::new()
            .with_position(0, 0)
            .with_wrap(WrapStyle::None)
            .with_segment(&TextSegment::new(&self.font, text, BLACK));

        // to accomodate the shadow under the text, add +1 to the dimensions
        let (mut text_width, mut text_height) = layout.dimensions();
        text_width += 1;
        text_height += 1;

        // this image will serve as a "stamp" to later apply to `text_image`
        let mut template_image = Image::new(text_width, text_height, TRANSPARENT);
        let mut text_image = Image::new(text_width, text_height, TRANSPARENT);

        // draw the shadow
        template_image.draw(&layout);
        // TODO: a lot of trial and error was involved in choosing this value for the default text;
        // this value and whether or not to pixelate the text should be configurable
        alpha_threshold(&mut template_image, 140);
        text_image.draw(&Paste {
            position: (1, 1),
            image: &template_image,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });

        // now map to the user-requested color and draw
        template_image.map_in_place(|_, _, p| {
            if *p != TRANSPARENT {
                // copy the RGB but leave the alpha matching the image above
                p.r = color.r;
                p.g = color.g;
                p.b = color.b;
            }
        });
        text_image.draw(&Paste {
            position: (0, 0),
            image: &template_image,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });

        text_image
    }
}

/// Applies an alpha threshold to an image in-place; pixels whose alpha value is
/// less-than-or-equal-to `cutoff` will become fully transparent, otherwise, they
/// will become fully opaque.
fn alpha_threshold(image: &mut Image<Rgba>, cutoff: u8) {
    image.map_in_place(|_, _, pixel| {
        if pixel.a <= cutoff {
            pixel.a = 0;
        } else {
            pixel.a = 255;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::TextRenderer;

    #[test]
    fn it_loads_default() {
        // no panic here means that this platform can load the default font
        TextRenderer::default();
    }
}
