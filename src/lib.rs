//! Deterministic pixel-art avatars from any seed.
//!
//! [`monster`] and [`nerd`] draw 8x8 faces, [`face`] picks one of the two by
//! the seed, [`crest`] draws a 10x10 shield with a division and an emblem,
//! and [`badge`] a 12x12 round badge with a division, an emblem and sometimes
//! a rim.
//! The seed is any byte string, typically a public user or group id.
//!
//! ```
//! let avatar = gsichtl::face(b"alice");
//! let image = avatar.to_rgba(12, 16);
//! assert_eq!(image.width, 8 * 12 + 2 * 16);
//! ```
//!
//! # Stability
//!
//! The same seed gives the same avatar within a `0.x` minor version. Changing
//! a part, a palette, the draw order or a context string is a breaking change
//! and bumps the minor version, because everyone who draws an avatar for the
//! same id has to see the same picture.
//!
//! # Not an identity check
//!
//! Anyone can upload a picture that looks like another person's generated
//! face. A generated avatar is a decoration; never present it as
//! verification of who someone is.

#![forbid(unsafe_code)]

mod badge;
mod crest;
mod draws;
mod grid;
mod monster;
mod nerd;

use draws::Draws;

/// An sRGB color.
pub type Rgb = [u8; 3];

const FACE_CONTEXT: &str = "gsichtl 2026-10-02 face v1";

/// What a generator drew.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Monster,
    Nerd,
    Crest,
    Badge,
}

/// A square grid of colored cells on a background.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Avatar {
    kind: Kind,
    side: u8,
    background: Rgb,
    /// Row-major; `None` shows the background.
    cells: Vec<Option<Rgb>>,
}

/// An RGBA8 image, row-major, with straight (not premultiplied) alpha.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

/// A monster or a nerd, chosen by the seed.
///
/// The choice consumes draws, so `face(seed)` is not necessarily
/// `monster(seed)` or `nerd(seed)` for the same seed.
pub fn face(seed: &[u8]) -> Avatar {
    let mut draws = Draws::new(FACE_CONTEXT, seed);
    if draws.pick(2) == 0 {
        monster::build(monster::draw(&mut draws))
    } else {
        nerd::build(nerd::draw(&mut draws))
    }
}

/// An 8x8 monster.
pub fn monster(seed: &[u8]) -> Avatar {
    monster::build(monster::draw(&mut Draws::new(monster::CONTEXT, seed)))
}

/// An 8x8 nerdy human face.
pub fn nerd(seed: &[u8]) -> Avatar {
    nerd::build(nerd::draw(&mut Draws::new(nerd::CONTEXT, seed)))
}

/// A 10x10 crest, for groups.
pub fn crest(seed: &[u8]) -> Avatar {
    crest::build(crest::draw(&mut Draws::new(crest::CONTEXT, seed)))
}

/// A 12x12 round badge, for groups.
pub fn badge(seed: &[u8]) -> Avatar {
    badge::build(badge::draw(&mut Draws::new(badge::CONTEXT, seed)))
}

impl Avatar {
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Cells per side.
    pub fn side(&self) -> u8 {
        self.side
    }

    pub fn background(&self) -> Rgb {
        self.background
    }

    /// The color of the cell at column `x`, row `y`; `None` where the
    /// background shows or outside the grid.
    pub fn cell(&self, x: u8, y: u8) -> Option<Rgb> {
        if x >= self.side || y >= self.side {
            return None;
        }
        let index = usize::from(y) * usize::from(self.side) + usize::from(x);
        self.cells.get(index).copied().flatten()
    }

    /// Renders every cell as a `cell_px` square inside a `margin_px` border of
    /// background. The image is `side * cell_px + 2 * margin_px` pixels wide
    /// and high, at most 3060. The image is opaque.
    pub fn to_rgba(&self, cell_px: u8, margin_px: u8) -> Rgba {
        self.render(cell_px, margin_px, Some(self.background))
    }

    /// [`Avatar::to_rgba`], but the background and the margin are fully
    /// transparent (`[0, 0, 0, 0]`); painted cells stay opaque.
    pub fn to_rgba_transparent(&self, cell_px: u8, margin_px: u8) -> Rgba {
        self.render(cell_px, margin_px, None)
    }

    /// [`Avatar::to_rgba`], encoded as PNG.
    #[cfg(feature = "png")]
    pub fn to_png(&self, cell_px: u8, margin_px: u8) -> Result<Vec<u8>, png::EncodingError> {
        encode_png(&self.to_rgba(cell_px, margin_px))
    }

    /// [`Avatar::to_rgba_transparent`], encoded as PNG.
    #[cfg(feature = "png")]
    pub fn to_png_transparent(
        &self,
        cell_px: u8,
        margin_px: u8,
    ) -> Result<Vec<u8>, png::EncodingError> {
        encode_png(&self.to_rgba_transparent(cell_px, margin_px))
    }

    /// Background and margin pixels are `background`, opaque, or fully
    /// transparent when it is `None`.
    fn render(&self, cell_px: u8, margin_px: u8, background: Option<Rgb>) -> Rgba {
        let empty = match background {
            Some([r, g, b]) => [r, g, b, u8::MAX],
            None => [0; 4],
        };
        let cell = u32::from(cell_px);
        let margin = u32::from(margin_px);
        let grid = u32::from(self.side) * cell;
        let side = grid + 2 * margin;
        let mut pixels = Vec::with_capacity(side as usize * side as usize * 4);
        for py in 0..side {
            for px in 0..side {
                let color = if (margin..margin + grid).contains(&px)
                    && (margin..margin + grid).contains(&py)
                {
                    // Both quotients are below `side`, a `u8`.
                    let x = u8::try_from((px - margin) / cell).unwrap_or(u8::MAX);
                    let y = u8::try_from((py - margin) / cell).unwrap_or(u8::MAX);
                    self.cell(x, y)
                } else {
                    None
                };
                match color {
                    Some([r, g, b]) => pixels.extend_from_slice(&[r, g, b, u8::MAX]),
                    None => pixels.extend_from_slice(&empty),
                }
            }
        }
        Rgba {
            width: side,
            height: side,
            pixels,
        }
    }
}

#[cfg(feature = "png")]
fn encode_png(image: &Rgba) -> Result<Vec<u8>, png::EncodingError> {
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, image.width, image.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image.pixels)?;
    writer.finish()?;
    Ok(out)
}
