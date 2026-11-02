//! 12x12 nerds: a head with hair, eyes or eyewear, a mouth, sometimes a beard and headphones.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE};
use crate::nerd::{FRAMES, HAIRS, SKINS};
use crate::{Avatar, Kind};

pub(crate) const CONTEXT: &str = "gsichtl 2026-11-02 nerd12 v1";
pub(crate) const SIDE: u8 = 12;

const HEADS: [Layer; 3] = [
    // plain
    Layer::new(
        1,
        &[
            "...######...",
            "..########..",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            "..########..",
            "...######...",
            "....####....",
        ],
        false,
    ),
    // ears
    Layer::new(
        1,
        &[
            "...######...",
            "..########..",
            ".##########.",
            ".##########.",
            ".##########.",
            "############",
            "############",
            ".##########.",
            "..########..",
            "...######...",
            "....####....",
        ],
        false,
    ),
    // square chin
    Layer::new(
        1,
        &[
            "...######...",
            "..########..",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            "..########..",
            "....####....",
        ],
        false,
    ),
];

const HAIR_STYLES: [Layer; 8] = [
    // short
    Layer::new(
        0,
        &[
            "............",
            "...hhhhhh...",
            "..hhhhhhhh..",
            ".hhhhhhhhhh.",
            ".hh......hh.",
        ],
        false,
    ),
    // spiky
    Layer::new(
        0,
        &[
            "..h.h..h.h..",
            "..hhhhhhhh..",
            ".hhhhhhhhhh.",
            ".hhhhhhhhhh.",
            ".h........h.",
        ],
        false,
    ),
    // side part
    Layer::new(
        0,
        &[
            "............",
            "...hhhhhh...",
            "..hhhhhhhh..",
            ".hhhhhhhhhh.",
            ".hhhhh...hh.",
        ],
        false,
    ),
    // long
    Layer::new(
        0,
        &[
            "............",
            "...hhhhhh...",
            "..hhhhhhhh..",
            ".hhhhhhhhhh.",
            "hhh......hhh",
            "hh........hh",
            "hh........hh",
            "hh........hh",
            "hh........hh",
            "h..........h",
        ],
        false,
    ),
    // beanie
    Layer::new(
        0,
        &[
            ".....aa.....",
            "...aaaaaa...",
            "..aaaaaaaa..",
            ".aaaaaaaaaa.",
            ".hh......hh.",
        ],
        false,
    ),
    // receding
    Layer::new(
        0,
        &[
            "............",
            "............",
            "..h......h..",
            ".hh......hh.",
            ".h........h.",
        ],
        false,
    ),
    // mohawk
    Layer::new(
        0,
        &[
            ".....hh.....",
            ".....hh.....",
            ".....hh.....",
            ".....hh.....",
        ],
        false,
    ),
    // afro
    Layer::new(
        0,
        &[
            "..hhhhhhhh..",
            ".hhhhhhhhhh.",
            "hhhhhhhhhhhh",
            "hhhhhhhhhhhh",
            "hhh......hhh",
            "hh........hh",
        ],
        false,
    ),
];

const EYES: [Layer; 5] = [
    // plain
    Layer::new(5, &["..www..www..", "..wpp..wpp.."], false),
    // glasses
    Layer::new(4, &[".gggg..gggg.", "ggwwpggwwpgg", ".gggg..gggg."], false),
    // shades
    Layer::new(5, &["gggggggggggg", ".gggg..gggg."], false),
    // goggles
    Layer::new(4, &["aaaaaaaaaaaa", "a.wwpaawwp.a", ".aaaa..aaaa."], false),
    // wink
    Layer::new(5, &["..www.......", "..wpp..ppp.."], false),
];

const MOUTHS: [Layer; 5] = [
    // smile
    Layer::new(8, &["..d......d..", "...dddddd..."], true),
    // flat
    Layer::new(9, &["....dddd...."], true),
    // buck teeth
    Layer::new(9, &["...dddddd...", ".....ww....."], true),
    // small
    Layer::new(9, &[".....dd....."], true),
    // grin
    Layer::new(9, &["...dwwwwd...", "....dddd...."], true),
];

const BEARD: Layer = Layer::new(
    7,
    &[
        ".h........h.",
        ".hh......hh.",
        "..hh....hh..",
        "...hhhhhh...",
        "....hhhh....",
    ],
    false,
);

const HEADPHONES: Layer = Layer::new(
    0,
    &[
        "...aaaaaa...",
        "..a......a..",
        ".a........a.",
        "a..........a",
        "a..........a",
        "aa........aa",
        "aa........aa",
        "aa........aa",
    ],
    false,
);

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&HEADS, &HAIR_STYLES, &EYES, &MOUTHS, &[BEARD, HEADPHONES]];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Nerd12Choice {
    bg: usize,
    skin: usize,
    hair: usize,
    frame: usize,
    accent: usize,
    head: usize,
    hair_style: usize,
    eyes: usize,
    mouth: usize,
    beard: bool,
    headphones: bool,
}

pub(crate) fn draw(draws: &mut Draws) -> Nerd12Choice {
    Nerd12Choice {
        bg: draws.pick(BACKGROUNDS.len()),
        skin: draws.pick(SKINS.len()),
        hair: draws.pick(HAIRS.len()),
        frame: draws.pick(FRAMES.len()),
        accent: draws.pick(ACCENTS.len()),
        head: draws.pick(HEADS.len()),
        hair_style: draws.pick(HAIR_STYLES.len()),
        eyes: draws.pick(EYES.len()),
        mouth: draws.pick(MOUTHS.len()),
        beard: draws.chance(20),
        headphones: draws.chance(25),
    }
}

pub(crate) fn build(choice: Nerd12Choice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let skin = SKINS[choice.skin % SKINS.len()];
    let hair = HAIRS[choice.hair % HAIRS.len()];
    let frame = FRAMES[choice.frame % FRAMES.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&HEADS[choice.head % HEADS.len()]);
    canvas.paint(&HAIR_STYLES[choice.hair_style % HAIR_STYLES.len()]);
    // The beard comes before the eyes and the mouth, so the mouth paints over it.
    if choice.beard {
        canvas.paint(&BEARD);
    }
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
