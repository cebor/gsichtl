//! Nerds: a head with hair, eyes or eyewear, a mouth and sometimes headphones.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-02 nerd v1";
pub(crate) const SIDE: u8 = 8;

const SKINS: [Rgb; 6] = [
    hex(0xffdbac),
    hex(0xf1c27d),
    hex(0xe0ac69),
    hex(0xc68642),
    hex(0x8d5524),
    hex(0x5c3a21),
];
const HAIRS: [Rgb; 8] = [
    hex(0x3b2a1e),
    hex(0x6f4e37),
    hex(0xb5651d),
    hex(0xe6c35c),
    hex(0x9e9e9e),
    hex(0x7c5cff),
    hex(0x4ecdc4),
    hex(0xff6b6b),
];
const FRAMES: [Rgb; 3] = [hex(0x11151c), hex(0xc92a2a), hex(0x1971c2)];

const HEADS: [Layer; 2] = [
    // plain
    Layer::new(
        1,
        &[
            "..####..", ".######.", ".######.", ".######.", ".######.", ".######.", "..####..",
        ],
        false,
    ),
    // ears
    Layer::new(
        1,
        &[
            "..####..", ".######.", ".######.", "########", ".######.", ".######.", "..####..",
        ],
        false,
    ),
];

const HAIR_STYLES: [Layer; 6] = [
    // short
    Layer::new(0, &["........", "..hhhh..", ".hhhhhh.", ".h....h."], false),
    // spiky
    Layer::new(0, &[".h.hh.h.", ".hhhhhh.", ".hhhhhh.", ".h....h."], false),
    // side part
    Layer::new(0, &["........", "..hhhh..", ".hhhhhh.", ".hhh..h."], false),
    // long
    Layer::new(
        0,
        &[
            "........", "..hhhh..", ".hhhhhh.", "hh....hh", "h......h", "h......h", "h......h",
        ],
        false,
    ),
    // beanie
    Layer::new(0, &["...aa...", "..aaaa..", ".aaaaaa.", ".h....h."], false),
    // receding
    Layer::new(0, &["........", "........", ".h....h.", ".h....h."], false),
];

const EYES: [Layer; 4] = [
    // plain
    Layer::new(4, &[".#w##w#.", ".#p##p#."], false),
    // glasses
    Layer::new(4, &["gwwggwwg", ".wp##pw."], false),
    // shades
    Layer::new(4, &["gggggggg", ".gg##gg."], false),
    // goggles
    Layer::new(4, &["aaaaaaaa", ".ap##pa."], false),
];

const MOUTHS: [Layer; 4] = [
    // smile
    Layer::new(6, &[".#d##d#.", "..#dd#.."], true),
    // flat
    Layer::new(6, &["..dddd.."], true),
    // buck teeth
    Layer::new(6, &["..dddd..", "...ww..."], true),
    // small
    Layer::new(6, &["...dd..."], true),
];

const HEADPHONES: Layer = Layer::new(
    0,
    &[
        "..aaaa..", ".a....a.", "a......a", "a......a", "a......a", "a......a",
    ],
    false,
);

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&HEADS, &HAIR_STYLES, &EYES, &MOUTHS, &[HEADPHONES]];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct NerdChoice {
    bg: usize,
    skin: usize,
    hair: usize,
    frame: usize,
    accent: usize,
    head: usize,
    hair_style: usize,
    eyes: usize,
    mouth: usize,
    headphones: bool,
}

pub(crate) fn draw(draws: &mut Draws) -> NerdChoice {
    NerdChoice {
        bg: draws.pick(BACKGROUNDS.len()),
        skin: draws.pick(SKINS.len()),
        hair: draws.pick(HAIRS.len()),
        frame: draws.pick(FRAMES.len()),
        accent: draws.pick(ACCENTS.len()),
        head: draws.pick(HEADS.len()),
        hair_style: draws.pick(HAIR_STYLES.len()),
        eyes: draws.pick(EYES.len()),
        mouth: draws.pick(MOUTHS.len()),
        headphones: draws.chance(25),
    }
}

pub(crate) fn build(choice: NerdChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let skin = SKINS[choice.skin % SKINS.len()];
    let hair = HAIRS[choice.hair % HAIRS.len()];
    let frame = FRAMES[choice.frame % FRAMES.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&HEADS[choice.head % HEADS.len()]);
    canvas.paint(&HAIR_STYLES[choice.hair_style % HAIR_STYLES.len()]);
    canvas.paint(&EYES[choice.eyes % EYES.len()]);
    canvas.paint(&MOUTHS[choice.mouth % MOUTHS.len()]);
    if choice.headphones {
        canvas.paint(&HEADPHONES);
    }
    canvas.finish(Kind::Nerd, background, |ink| match ink {
        Ink::Dark | Ink::Pupil => PUPIL,
        Ink::White => WHITE,
        Ink::Hair => hair,
        Ink::Frame => frame,
        Ink::Accent => accent,
        Ink::Body | Ink::Field2 | Ink::Metal => skin,
    })
}
