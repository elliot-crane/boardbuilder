use ril::Rgba;

pub const BLACK: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const TRANSPARENT: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const YELLOW: Rgba = Rgba {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};
pub const ORANGE: Rgba = Rgba {
    r: 255,
    g: 144,
    b: 0,
    a: 255,
};
pub const GREEN: Rgba = Rgba {
    r: 0,
    g: 255,
    b: 28,
    a: 255,
};

// board-specific colors
pub const DEFAULT_BORDER_COLOR: Rgba = Rgba {
    r: 47,
    g: 43,
    b: 34,
    a: 255,
};
pub const DEFAULT_INSET_COLOR: Rgba = Rgba {
    r: 117,
    g: 99,
    b: 78,
    a: 255,
};
pub const DEFAULT_BACKGROUND_LOCKED_COLOR: Rgba = Rgba {
    r: 74,
    g: 62,
    b: 50,
    a: 255,
};
pub const DEFAULT_BACKGROUND_UNLOCKED_COLOR: Rgba = Rgba {
    r: 87,
    g: 76,
    b: 64,
    a: 255,
};
