//! This module contains (de)serializable primitives that can be turned into boards.

use std::collections::HashSet;

use crate::{
    board::Board,
    error::AppError,
    images::ImageLoader,
    tile::{Tile, TileRenderOptions},
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoardBuilderError {
    #[error(transparent)]
    Wrapped(AppError),

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

#[derive(Deserialize, Debug)]
pub struct TileBuilder {
    pub number: u8,
    pub name: String,
    pub image: String,
    pub unlocked: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ContentRect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(Deserialize, Debug)]
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

        let background_image = image_loader
            .load(&image)
            .map_err(BoardBuilderError::Wrapped)?;
        validate_content_rect(
            background_image.dimensions(),
            &content_rect,
            tile_size,
            rows,
            cols,
        )?;

        // build tiles
        let tiles = build_tiles(&tiles, image_loader).map_err(BoardBuilderError::Wrapped)?;

        // shadowing to make the syntax below a bit neater
        let image = background_image;
        let content_rect = (
            content_rect.x1,
            content_rect.y1,
            content_rect.x2,
            content_rect.y2,
        );

        Ok(Board {
            rows,
            cols,
            content_rect,
            tile_size,
            tile_render_options,
            tiles,
            image,
        })
    }
}

fn build_tiles(tiles: &[TileBuilder], image_loader: &ImageLoader) -> Result<Vec<Tile>, AppError> {
    let mut result = Vec::with_capacity(tiles.len());
    for builder in tiles.iter() {
        let number = builder.number;
        let name = builder.name.clone();
        let image = image_loader.load(&builder.image)?;
        let unlocked = builder.unlocked;
        let tile = Tile {
            number,
            name,
            image,
            unlocked,
        };
        result.push(tile);
    }
    Ok(result)
}

fn validate_content_rect(
    dimensions: (u32, u32),
    content_rect: &ContentRect,
    tile_size: u32,
    rows: usize,
    cols: usize,
) -> Result<(), BoardBuilderError> {
    // TODO: better information on what's actually wrong
    let (width, height) = dimensions;
    if content_rect.x1 >= content_rect.x2 || content_rect.y1 >= content_rect.y2 {
        return Err(BoardBuilderError::InvalidDimensions {
            width,
            height,
            content_rect: content_rect.clone(),
        });
    }
    let rect_width = content_rect.x2 - content_rect.x1;
    let rect_height = content_rect.y2 - content_rect.y1;
    if rect_width > width || rect_height > height {
        return Err(BoardBuilderError::InvalidDimensions {
            width,
            height,
            content_rect: content_rect.clone(),
        });
    }
    let tile_width = tile_size * cols as u32;
    let tile_height = tile_size * rows as u32;
    if tile_width > rect_width || tile_height > rect_height {
        return Err(BoardBuilderError::InvalidDimensions {
            width,
            height,
            content_rect: content_rect.clone(),
        });
    }
    Ok(())
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
    let mut missing = (1..=tiles.len()).map(|x| x as u8).collect::<HashSet<_>>();
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
