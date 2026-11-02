//! Robots: a metal head with something on top, glowing eyes, a mouth and sometimes bolts.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-11-02 robot v1";
pub(crate) const SIDE: u8 = 8;

/// (body, trim): the trim is the darker shade, for bolts and antenna stalks.
const HULLS: [(Rgb, Rgb); 8] = [
    (hex(0xadb5bd), hex(0x6c757d)), // steel
    (hex(0x5c677d), hex(0x3a4252)), // gunmetal
    (hex(0x7f9cb5), hex(0x4a6a85)), // chrome blue
    (hex(0xa47148), hex(0x6b4423)), // bronze
    (hex(0x8a9a5b), hex(0x5a6638)), // olive
    (hex(0x8e7cc3), hex(0x5b4a8a)), // slate purple
    (hex(0xc2b280), hex(0x8a7b55)), // sand
    (hex(0x5f9ea0), hex(0x3a6b6d)), // teal
];

/// LED colors for eyes and glowing mouths.
const GLOWS: [Rgb; 5] = [
    hex(0x22e6ff), // cyan
    hex(0xff3b3b), // red
    hex(0x5cff6e), // green
    hex(0xffb02e), // amber
    hex(0xff4fd8), // magenta
];

const HEADS: [Layer; 4] = [
    // box
    Layer::new(
        2,
        &[
            ".######.", ".######.", ".######.", ".######.", ".######.", ".######.",
        ],
        false,
    ),
    // dome
    Layer::new(
        2,
        &[
            "..####..", ".######.", ".######.", ".######.", ".######.", ".######.",
        ],
        false,
    ),
    // tv
    Layer::new(
        2,
        &[
            "########", "########", "########", "########", "########", "########",
        ],
        false,
    ),
    // jaw
    Layer::new(
        2,
        &[
            ".######.", ".######.", ".######.", ".######.", "..####..", "..####..",
        ],
        false,
    ),
];

const TOPS: [Layer; 5] = [
    // antenna
    Layer::new(0, &["...a....", "...e...."], false),
    // twin antennae
    Layer::new(0, &[".a....a.", ".e....e."], false),
    // siren
    Layer::new(0, &["...aa...", "..eeee.."], false),
    // propeller
    Layer::new(0, &["aaaeeaaa", "...ee..."], false),
    // none
    Layer::new(0, &["........", "........"], false),
];

const EYES: [Layer; 5] = [
    // visor
    Layer::new(3, &["dddddddd", "dppppppd"], true),
    // cyclops lens
    Layer::new(3, &["##dddd##", "##dppd##"], true),
    // sockets
    Layer::new(3, &[".dp##pd.", ".dd##dd."], true),
    // square
    Layer::new(3, &[".pp##pp.", ".pp##pp."], true),
    // slits
    Layer::new(3, &["########", ".pp##pp."], true),
];

const MOUTHS: [Layer; 4] = [
    // slot
    Layer::new(6, &["##dddd##"], true),
    // teeth
    Layer::new(6, &["#wdwwdw#"], true),
    // glowing smile
    Layer::new(6, &["#p####p#", "##pppp##"], true),
    // speaker
    Layer::new(6, &["###dd###"], true),
];

const SIDES: [Layer; 3] = [
    // bolts
    Layer::new(4, &["e......e", "e......e"], false),
    // ear lights
    Layer::new(4, &["a......a", "e......e"], false),
    // none
    Layer::new(4, &[], false),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&HEADS, &TOPS, &EYES, &MOUTHS, &SIDES];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RobotChoice {
    bg: usize,
    hull: usize,
    glow: usize,
    accent: usize,
    head: usize,
    top: usize,
    eyes: usize,
    mouth: usize,
    side: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> RobotChoice {
    RobotChoice {
        bg: draws.pick(BACKGROUNDS.len()),
        hull: draws.pick(HULLS.len()),
        glow: draws.pick(GLOWS.len()),
        accent: draws.pick(ACCENTS.len()),
        head: draws.pick(HEADS.len()),
        top: draws.pick(TOPS.len()),
        eyes: draws.pick(EYES.len()),
        mouth: draws.pick(MOUTHS.len()),
        side: draws.pick(SIDES.len()),
    }
}

pub(crate) fn build(choice: RobotChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let (body, trim) = HULLS[choice.hull % HULLS.len()];
    let glow = GLOWS[choice.glow % GLOWS.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&HEADS[choice.head % HEADS.len()]);
    canvas.paint(&TOPS[choice.top % TOPS.len()]);
    canvas.paint(&EYES[choice.eyes % EYES.len()]);
    canvas.paint(&MOUTHS[choice.mouth % MOUTHS.len()]);
    // Sides are not clipped: on the tv head their bolts cover the edge
    // cells of the visor and the mouth.
    canvas.paint(&SIDES[choice.side % SIDES.len()]);
    canvas.finish(Kind::Robot, background, |ink| match ink {
        // A black visor reads as glass, not as a hole.
        Ink::Dark => PUPIL,
        // A robot's pupils are its LEDs.
        Ink::Pupil => glow,
        Ink::White => WHITE,
        Ink::Accent => accent,
        Ink::Metal => trim,
        Ink::Body | Ink::Hair | Ink::Frame | Ink::Field2 => body,
    })
}
