//! This module contains (de)serializable primitives that can be turned into boards.

use std::collections::HashSet;

use crate::{board::Board, images::ImageLoader, tile::TileRenderOptions};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoardBuilderError {
    #[error("wrong number of tiles: expected {expected:?}, actual {actual:?}")]
    WrongNumberOfTiles { expected: usize, actual: usize },

    #[error("tiles must be consecutively numbered, missing {0:?}")]
    MissingTiles(HashSet<u8>),

    #[error("tiles must be consecutively numbered, unexpected {0:?}")]
    UnexpectedTiles(HashSet<u8>),

    #[error("invalid dimensions: {width:?}px x {height:?}px image cannot support content rectangle {content_rect:?}")]
    InvalidDimensions {
        width: u32,
        height: u32,
        content_rect: ContentRect,
    },
}

pub struct TileBuilder {
    pub number: u8,
    pub name: String,
    pub image: String,
    pub unlocked: bool,
}

#[derive(Clone, Debug)]
pub struct ContentRect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

pub struct BoardBuilder {
    pub rows: usize,
    pub cols: usize,
    pub content_rect: ContentRect,
    pub tile_size: u32,
    pub tile_render_options: TileRenderOptions,
    pub image: String,
    pub tiles: Vec<TileBuilder>,
}

impl BoardBuilder {
    pub fn build(self, image_loader: &ImageLoader) -> Result<Board, BoardBuilderError> {
        let BoardBuilder {
            rows,
            cols,
            content_rect,
            tile_size,
            tile_render_options,
            image,
            mut tiles,
        } = self;

        // tile validation stuff
        tiles.sort_by(|a, b| a.number.cmp(&b.number));
        validate_tile_count(rows as usize, cols as usize, &tiles)?;
        validate_tile_numbers(&tiles)?;

        todo!()
    }
}

fn validate_tile_count(
    rows: usize,
    cols: usize,
    tiles: &[TileBuilder],
) -> Result<(), BoardBuilderError> {
    let expected = rows * cols;
    let actual = tiles.len();
    if expected != actual {
        return Err(BoardBuilderError::WrongNumberOfTiles { expected, actual });
    }
    Ok(())
}

fn validate_tile_numbers(tiles: &[TileBuilder]) -> Result<(), BoardBuilderError> {
    let mut missing = (1..=(1 + tiles.len()))
        .map(|x| x as u8)
        .collect::<HashSet<_>>();
    let mut unexpected = HashSet::new();
    for tile in tiles {
        if !missing.remove(&tile.number) {
            unexpected.insert(tile.number);
        }
    }
    if !unexpected.is_empty() {
        return Err(BoardBuilderError::UnexpectedTiles(unexpected));
    }
    if !missing.is_empty() {
        return Err(BoardBuilderError::MissingTiles(missing));
    }
    Ok(())
}
