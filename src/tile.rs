use ril::{Border, BorderPosition, Image, OverlayMode, Paste, Rectangle, ResizeAlgorithm, Rgba};

use crate::{
    palette::{
        DEFAULT_BACKGROUND_LOCKED_COLOR, DEFAULT_BACKGROUND_UNLOCKED_COLOR, DEFAULT_BORDER_COLOR,
        DEFAULT_INSET_COLOR, GREEN, ORANGE,
    },
    text::{TextRenderOptions, TextRenderer},
};

pub struct Tile {
    pub number: u8,
    pub name: String,
    pub image: Image<Rgba>,
    pub unlocked: bool,
}

pub struct TileRenderOptions {
    pub size: u32,
    pub padding: u32,
    pub border_size: u32,
    pub inset_size: u32,
    pub text_size: u32,
    pub locked_theme: TileTheme,
    pub unlocked_theme: TileTheme,
}

impl Default for TileRenderOptions {
    fn default() -> Self {
        Self {
            size: 216,
            padding: 6,
            border_size: 4,
            inset_size: 4,
            text_size: 20,
            locked_theme: TileTheme {
                border_color: DEFAULT_BORDER_COLOR,
                inset_color: DEFAULT_INSET_COLOR,
                background_color: DEFAULT_BACKGROUND_LOCKED_COLOR,
                text_color: ORANGE,
            },
            unlocked_theme: TileTheme {
                border_color: DEFAULT_BORDER_COLOR,
                inset_color: DEFAULT_INSET_COLOR,
                background_color: DEFAULT_BACKGROUND_UNLOCKED_COLOR,
                text_color: GREEN,
            },
        }
    }
}

pub struct TileTheme {
    pub border_color: Rgba,
    pub inset_color: Rgba,
    pub background_color: Rgba,
    pub text_color: Rgba,
}

pub struct TileRenderer<'a> {
    options: TileRenderOptions,
    /// x1, y1, x2, y2 rectangle that accounts for border, inset, and padding
    content_bounds: (u32, u32, u32, u32),
    locked_template: Image<Rgba>,
    unlocked_template: Image<Rgba>,
    text_renderer: &'a TextRenderer,
}

impl<'a> TileRenderer<'a> {
    pub fn new(options: TileRenderOptions, text_renderer: &'a TextRenderer) -> Self {
        let content_bounds = compute_content_bounds(&options);
        let locked_template = render_tile_template(
            options.size,
            options.border_size,
            options.inset_size,
            options.locked_theme.background_color,
            options.locked_theme.border_color,
            options.locked_theme.inset_color,
        );
        let unlocked_template = render_tile_template(
            options.size,
            options.border_size,
            options.inset_size,
            options.unlocked_theme.background_color,
            options.unlocked_theme.border_color,
            options.unlocked_theme.inset_color,
        );
        Self {
            options,
            content_bounds,
            locked_template,
            unlocked_template,
            text_renderer,
        }
    }

    pub fn render(&self, tile: &Tile) -> Image<Rgba> {
        let mut image = if tile.unlocked {
            self.unlocked_template.clone()
        } else {
            self.locked_template.clone()
        };
        let theme = if tile.unlocked {
            &self.options.unlocked_theme
        } else {
            &self.options.locked_theme
        };
        let (x1, mut y1, x2, mut y2) = self.content_bounds;
        let text_size = self.options.text_size as f32;
        // composite in text
        let number_text = self.text_renderer.render(
            &tile.number.to_string(),
            &TextRenderOptions {
                size: text_size,
                color: theme.text_color,
                pixelation: None,
            },
        );
        let name_text = self.text_renderer.render(
            &tile.name,
            &TextRenderOptions {
                size: text_size,
                color: theme.text_color,
                pixelation: None,
            },
        );
        image.draw(&Paste {
            position: (x1, y1),
            image: &number_text,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });
        let content_width = x2 - x1;
        let x_offset = if name_text.width() < content_width {
            (content_width - name_text.width()) / 2
        } else {
            // TODO: text is too large for the tile - panic here?
            0
        };
        image.draw(&Paste {
            position: (x1 + x_offset, y2 - name_text.height()),
            image: &name_text,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });
        // now shift y1 and y2 so that the tile's image does not overlap the text
        y1 += number_text.height() + self.options.padding;
        y2 -= name_text.height() + self.options.padding;
        let content_width = x2 - x1;
        let content_height = y2 - y1;
        let mut item_image = tile.image.clone();
        // resize image if necessary
        if item_image.width() > content_width || item_image.height() > content_height {
            let new_width;
            let new_height;
            if item_image.width() > item_image.height() {
                let scale_factor = item_image.width() as f32 / content_width as f32;
                new_width = content_width;
                new_height = (item_image.height() as f32 / scale_factor).floor() as u32;
            } else {
                let scale_factor = item_image.height() as f32 / content_height as f32;
                new_width = (item_image.width() as f32 / scale_factor).floor() as u32;
                new_height = content_height;
            }
            item_image.resize(new_width, new_height, ResizeAlgorithm::Bicubic);
        }
        // locked tiles are grayed out
        if !tile.unlocked {
            desaturate(&mut item_image, 0.9);
        }
        let x_pad = (content_width - item_image.width()) / 2;
        let y_pad = (content_height - item_image.height()) / 2;
        image.draw(&Paste {
            position: (x1 + x_pad, y1 + y_pad),
            image: &item_image,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });
        image
    }
}

fn compute_content_bounds(options: &TileRenderOptions) -> (u32, u32, u32, u32) {
    let offset = options.border_size + options.inset_size + options.padding;
    let x2 = options.size - offset;
    let y2 = options.size - offset;
    (offset, offset, x2, y2)
}

fn render_tile_template(
    size: u32,
    border_size: u32,
    inset_size: u32,
    background_color: Rgba,
    border_color: Rgba,
    inset_color: Rgba,
) -> Image<Rgba> {
    let mut image = Image::new(size, size, background_color);
    let border = Rectangle::<Rgba>::from_bounding_box(0, 0, size, size)
        .with_border(Border::new(border_color, border_size).with_position(BorderPosition::Inset));
    let inset = Rectangle::<Rgba>::from_bounding_box(
        border_size,
        border_size,
        size - border_size,
        size - border_size,
    )
    .with_border(Border::new(inset_color, inset_size).with_position(BorderPosition::Inset));
    image.draw(&border);
    image.draw(&inset);
    image
}

fn desaturate(image: &mut Image<Rgba>, factor: f32) {
    // borrowed this approximation from SO: https://stackoverflow.com/a/20820649
    image.map_in_place(|_, _, p| {
        let r = p.r as f32;
        let g = p.g as f32;
        let b = p.b as f32;
        let luma = 0.3 * r + 0.6 * g + 0.1 * b;
        p.r = (r + factor * (luma - r)).floor().clamp(0.0, 255.0) as u8;
        p.g = (g + factor * (luma - g)).floor().clamp(0.0, 255.0) as u8;
        p.b = (b + factor * (luma - b)).floor().clamp(0.0, 255.0) as u8;
    });
}
