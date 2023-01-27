use ril::{Font, Image, OverlayMode, Paste, Rgba, TextLayout, TextSegment, WrapStyle};

use crate::{
    error::AppError,
    palette::{BLACK, TRANSPARENT, YELLOW},
};

const DEFAULT_FONT_BYTES: &[u8] =
    include_bytes!("../assets/fonts/runescape-chat-bold-2/runescape-chat-bold-2.otf");

pub struct TextRenderer {
    font: Font,
}

/// Configures the text renderer
pub struct TextRenderOptions {
    /// The font size
    pub size: f32,
    /// Color to render with
    pub color: Rgba,
    /// Options for pixelating to undo built-in antialiasing
    pub pixelation: Option<TextPixelationOptions>,
}

impl Default for TextRenderOptions {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: YELLOW,
            pixelation: None,
        }
    }
}

/// Options for pixelating text
pub struct TextPixelationOptions {
    /// Antialiased pixels less-than or equal-to this alpha value will become
    /// completely transparent; any other pixels will become completely opaque
    pub alpha_threshold: u8,
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

    pub fn render(&self, text: impl AsRef<str>, options: &TextRenderOptions) -> Image<Rgba> {
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
        if let Some(pixelation) = &options.pixelation {
            alpha_threshold(&mut template_image, pixelation.alpha_threshold);
        }
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
                p.r = options.color.r;
                p.g = options.color.g;
                p.b = options.color.b;
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
