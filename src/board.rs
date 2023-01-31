use ril::{Image, OverlayMode, Paste, Rgba};

use crate::{
    palette::TRANSPARENT,
    tile::{Tile, TileRenderOptions, TileRenderer},
};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    /// (x1, y1, x2, y2) rectangle of where tiles can be drawn
    pub content_rect: (u32, u32, u32, u32),
    /// how long or wide a tile is
    pub tile_size: u32,
    pub image: Image<Rgba>,
    pub tiles: Vec<Tile>,
}

pub struct BoardRenderer<'a> {
    tile_renderer: &'a TileRenderer<'a>,
}

impl<'a> BoardRenderer<'a> {
    pub fn new(tile_renderer: &'a TileRenderer) -> Self {
        Self { tile_renderer }
    }

    pub fn render(&self, board: &Board) -> Image<Rgba> {
        let mut image = Image::new(board.image.width(), board.image.height(), TRANSPARENT);
        // start by compositing the board image onto the output image
        image.draw(&Paste {
            position: (0, 0),
            image: &board.image,
            mask: None,
            overlay: Some(OverlayMode::Merge),
        });
        // now compute the spacings for tiles
        let (x1, y1, x2, y2) = board.content_rect;
        let content_width = x2 - x1;
        let content_height = y2 - y1;
        let cols = board.cols as u32;
        let rows = board.rows as u32;
        let tile_size = board.tile_size;
        let tiles_width = cols * tile_size;
        let tiles_height = rows * tile_size;
        let x_pad = (content_width - tiles_width) / cols;
        let y_pad = (content_height - tiles_height) / rows;
        // place tiles
        let mut x = x1;
        let mut y = y1;
        for row in board.tiles.chunks(board.cols) {
            for tile in row {
                // TODO: customizable theme
                let tile_image = self
                    .tile_renderer
                    .render(tile, &TileRenderOptions::default());
                image.draw(&Paste {
                    position: (x, y),
                    image: &tile_image,
                    mask: None,
                    overlay: Some(OverlayMode::Merge),
                });
                x += tile_size + x_pad;
            }
            y += tile_size + y_pad;
        }
        image
    }
}
