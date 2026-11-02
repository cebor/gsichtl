//! 12x12 robots: a metal head with something on top, glowing eyes, a mouth and sometimes bolts.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE};
use crate::robot::{GLOWS, HULLS};
use crate::{Avatar, Kind};

pub(crate) const CONTEXT: &str = "gsichtl 2026-11-02 robot12 v1";
pub(crate) const SIDE: u8 = 12;

const HEADS: [Layer; 5] = [
    // box
    Layer::new(
        3,
        &[
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
        ],
        false,
    ),
    // dome
    Layer::new(
        3,
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
        ],
        false,
    ),
    // tv
    Layer::new(
        3,
        &[
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
        ],
        false,
    ),
    // jaw
    Layer::new(
        3,
        &[
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            "..########..",
            "..########..",
            "...######...",
        ],
        false,
    ),
    // capsule
    Layer::new(
        3,
        &[
            "..########..",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            ".##########.",
            "..########..",
        ],
        false,
    ),
];

const TOPS: [Layer; 6] = [
    // antenna
    Layer::new(0, &[".....aa.....", ".....ee.....", ".....ee....."], false),
    // twin antennae
    Layer::new(0, &["..a......a..", "..e......e..", "..e......e.."], false),
    // siren
    Layer::new(0, &[".....aa.....", "....aaaa....", "...eeeeee..."], false),
    // propeller
    Layer::new(0, &["aaaaaeeaaaaa", ".....ee.....", ".....ee....."], false),
    // rabbit ears
    Layer::new(0, &["a..........a", ".e........e.", "..e......e.."], false),
    // none
    Layer::new(0, &[], false),
];

const EYES: [Layer; 6] = [
    // visor
    Layer::new(5, &["dddddddddddd", "dppppppppppd", "dddddddddddd"], true),
    // cyclops lens
    Layer::new(5, &["####dddd####", "###ddppdd###", "####dddd####"], true),
    // sockets
    Layer::new(5, &["#dddd##dddd#", "#dppd##dppd#", "#dddd##dddd#"], true),
    // square
    Layer::new(5, &["#ppp####ppp#", "#ppp####ppp#"], true),
    // slits
    Layer::new(6, &["#pppp##pppp#"], true),
    // scanner
    Layer::new(5, &["dddddddddddd", "dddpppdddddd", "dddddddddddd"], true),
];

const MOUTHS: [Layer; 5] = [
    // slot
    Layer::new(9, &["###dddddd###"], true),
    // teeth
    Layer::new(9, &["##wdwddwdw##", "##dddddddd##"], true),
    // glowing smile
    Layer::new(9, &["#p########p#", "##pppppppp##"], true),
    // speaker
    Layer::new(9, &["##d#d##d#d##", "##d#d##d#d##"], true),
    // zigzag
    Layer::new(9, &["##p##pp##p##", "###pp##pp###"], true),
];

const SIDES: [Layer; 4] = [
    // bolts
    Layer::new(5, &["e..........e", "e..........e", "e..........e"], false),
    // ear lights
    Layer::new(5, &["a..........a", "e..........e", "e..........e"], false),
    // rivets
    Layer::new(
        4,
        &[
            ".e........e.",
            "............",
            "............",
            "............",
            "............",
            "............",
            "............",
            ".e........e.",
        ],
        true,
    ),
    // none
    Layer::new(5, &[], false),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&HEADS, &TOPS, &EYES, &MOUTHS, &SIDES];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Robot12Choice {
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

pub(crate) fn draw(draws: &mut Draws) -> Robot12Choice {
    Robot12Choice {
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

pub(crate) fn build(choice: Robot12Choice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let (body, trim) = HULLS[choice.hull % HULLS.len()];
    let glow = GLOWS[choice.glow % GLOWS.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&HEADS[choice.head % HEADS.len()]);
    canvas.paint(&TOPS[choice.top % TOPS.len()]);
    canvas.paint(&EYES[choice.eyes % EYES.len()]);
    canvas.paint(&MOUTHS[choice.mouth % MOUTHS.len()]);
    // Bolts and ear lights are not clipped: on the tv head they cover the edge
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
