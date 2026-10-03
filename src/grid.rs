//! The cell grid the generators paint their parts on.

use crate::{Avatar, Kind, Rgb};

/// The role of a painted cell; each generator maps it to a color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ink {
    Body,
    Dark,
    White,
    Pupil,
    Accent,
    Hair,
    Frame,
    Field2,
    Metal,
}

impl Ink {
    /// The ink a part character stands for; `None` leaves the cell alone.
    fn from_byte(byte: u8) -> Option<Self> {
        Some(match byte {
            b'#' => Self::Body,
            b'd' => Self::Dark,
            b'w' => Self::White,
            b'p' => Self::Pupil,
            b'a' => Self::Accent,
            b'h' => Self::Hair,
            b'g' => Self::Frame,
            b't' => Self::Field2,
            b'e' => Self::Metal,
            _ => return None,
        })
    }
}

/// A part: rows of ink characters placed at a cell offset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Layer {
    pub(crate) top: usize,
    pub(crate) left: usize,
    pub(crate) rows: &'static [&'static str],
    /// Paint only cells something below has painted already, so the part
    /// stays inside the silhouette.
    pub(crate) clip: bool,
}

impl Layer {
    pub(crate) const fn new(top: usize, rows: &'static [&'static str], clip: bool) -> Self {
        Self {
            top,
            left: 0,
            rows,
            clip,
        }
    }

    pub(crate) const fn at(
        top: usize,
        left: usize,
        rows: &'static [&'static str],
        clip: bool,
    ) -> Self {
        Self {
            top,
            left,
            rows,
            clip,
        }
    }
}

pub(crate) struct Canvas {
    side: usize,
    cells: Vec<Option<Ink>>,
}

impl Canvas {
    pub(crate) fn new(side: u8) -> Self {
        let side = usize::from(side);
        Self {
            side,
            cells: vec![None; side * side],
        }
    }

    pub(crate) fn paint(&mut self, layer: &Layer) {
        for (dy, row) in layer.rows.iter().enumerate() {
            let y = layer.top + dy;
            for (dx, byte) in row.bytes().enumerate() {
                let x = layer.left + dx;
                let Some(ink) = Ink::from_byte(byte) else {
                    continue;
                };
                if x >= self.side || y >= self.side {
                    continue;
                }
                let Some(cell) = self.cells.get_mut(y * self.side + x) else {
                    continue;
                };
                if !layer.clip || cell.is_some() {
                    *cell = Some(ink);
                }
            }
        }
    }

    pub(crate) fn finish(self, kind: Kind, background: Rgb, color: impl Fn(Ink) -> Rgb) -> Avatar {
        Avatar {
            kind,
            // The canvas was made from a `u8` side.
            side: u8::try_from(self.side).unwrap_or(u8::MAX),
            background,
            cells: self.cells.into_iter().map(|ink| ink.map(&color)).collect(),
        }
    }
}

/// A color from its `0xrrggbb` value.
pub(crate) const fn hex(value: u32) -> Rgb {
    [(value >> 16) as u8, (value >> 8) as u8, value as u8]
}

pub(crate) const BACKGROUNDS: [Rgb; 4] =
    [hex(0x1a1530), hex(0x13263a), hex(0x2a1630), hex(0x1c2a22)];
pub(crate) const WHITE: Rgb = hex(0xffffff);
pub(crate) const PUPIL: Rgb = hex(0x11151c);
pub(crate) const ACCENTS: [Rgb; 6] = [
    hex(0xffd166),
    hex(0xf8f9fa),
    hex(0xff8787),
    hex(0x8ce99a),
    hex(0x74c0fc),
    hex(0xffc078),
];

#[cfg(test)]
mod tests {
    use super::Layer;

    fn check(name: &str, side: u8, parts: &[&[Layer]]) {
        let side = usize::from(side);
        for (slot, layers) in parts.iter().enumerate() {
            for (variant, layer) in layers.iter().enumerate() {
                let at = format!("{name} slot {slot} variant {variant}");
                assert!(layer.top + layer.rows.len() <= side, "{at}: too tall");
                let width = layer.rows.first().map_or(0, |row| row.len());
                for row in layer.rows {
                    assert_eq!(row.len(), width, "{at}: ragged row {row:?}");
                    assert!(
                        row.bytes().all(|b| b".#dwpahgte".contains(&b)),
                        "{at}: unknown character in {row:?}"
                    );
                }
                assert!(layer.left + width <= side, "{at}: too wide");
            }
        }
    }

    #[test]
    fn parts_are_well_formed() {
        check("monster", crate::monster::SIDE, crate::monster::PARTS);
        check("nerd", crate::nerd::SIDE, crate::nerd::PARTS);
        check("crest", crate::crest::SIDE, crate::crest::PARTS);
        check("badge", crate::badge::SIDE, crate::badge::PARTS);
    }
}
