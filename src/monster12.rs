//! 12x12 monsters: a body silhouette with something on top, brows, eyes and a mouth.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE};
use crate::monster::BODIES;
use crate::{Avatar, Kind};

pub(crate) const CONTEXT: &str = "gsichtl 2026-11-02 monster12 v1";
pub(crate) const SIDE: u8 = 12;

const SILHOUETTES: [Layer; 4] = [
    // square
    Layer::new(
        3,
        &[
            ".##########.",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            ".##########.",
        ],
        false,
    ),
    // round
    Layer::new(
        3,
        &[
            "...######...",
            ".##########.",
            ".##########.",
            "############",
            "############",
            "############",
            ".##########.",
            ".##########.",
            "...######...",
        ],
        false,
    ),
    // bell
    Layer::new(
        3,
        &[
            "..########..",
            ".##########.",
            ".##########.",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
        ],
        false,
    ),
    // blob with feet
    Layer::new(
        3,
        &[
            "..########..",
            ".##########.",
            "############",
            "############",
            "############",
            "############",
            "############",
            "############",
            ".###....###.",
        ],
        false,
    ),
];

const TOPS: [Layer; 7] = [
    // antennae
    Layer::new(0, &["..a......a..", "..#......#..", "...#....#..."], false),
    // horns
    Layer::new(0, &["a..........a", "aa........aa", ".aa......aa."], false),
    // ears
    Layer::new(0, &[".##......##.", ".###....###.", ".####..####."], false),
    // bobble
    Layer::new(0, &[".....aa.....", ".....aa.....", ".....##....."], false),
    // spikes
    Layer::new(0, &["............", "..#..##..#..", ".###.##.###."], false),
    // crown
    Layer::new(0, &["..a..aa..a..", "..aa.aa.aa..", "..aaaaaaaa.."], false),
    // none
    Layer::new(0, &[], false),
];

const BROWS: [Layer; 4] = [
    // slot
    Layer::new(4, &["...dddddd..."], true),
    // none
    Layer::new(4, &[], true),
    // brows
    Layer::new(4, &[".ddd....ddd."], true),
    // unibrow
    Layer::new(4, &["..dddddddd.."], true),
];

const EYES: [Layer; 6] = [
    // looking right
    Layer::new(5, &["#www####www#", "#wpp####wpp#", "#wpp####wpp#"], true),
    // looking left
    Layer::new(5, &["#www####www#", "#ppw####ppw#", "#ppw####ppw#"], true),
    // looking up
    Layer::new(5, &["#wpw####wpw#", "#wpw####wpw#", "#www####www#"], true),
    // cyclops
    Layer::new(5, &["####wwww####", "###wwppww###", "####wwww####"], true),
    // sleepy
    Layer::new(5, &["#ddd####ddd#", "#wpp####wpp#", "#www####www#"], true),
    // knocked out
    Layer::new(5, &["#d#d####d#d#", "##d######d##", "#d#d####d#d#"], true),
];

const MOUTHS: [Layer; 6] = [
    // teeth
    Layer::new(9, &["..wdwddwdw..", "..dddddddd.."], true),
    // smile
    Layer::new(9, &["..d......d..", "...dddddd..."], true),
    // fangs
    Layer::new(9, &["..dddddddd..", "...w....w..."], true),
    // small
    Layer::new(9, &["....dddd...."], true),
    // tongue
    Layer::new(9, &["..dddddddd..", "....aaaa...."], true),
    // none
    Layer::new(9, &[], true),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&SILHOUETTES, &TOPS, &BROWS, &EYES, &MOUTHS];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Monster12Choice {
    bg: usize,
    body: usize,
    accent: usize,
    top: usize,
    silhouette: usize,
    brow: usize,
    eyes: usize,
    mouth: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> Monster12Choice {
    Monster12Choice {
        bg: draws.pick(BACKGROUNDS.len()),
        body: draws.pick(BODIES.len()),
        accent: draws.pick(ACCENTS.len()),
        top: draws.pick(TOPS.len()),
        silhouette: draws.pick(SILHOUETTES.len()),
        brow: draws.pick(BROWS.len()),
        eyes: draws.pick(EYES.len()),
        mouth: draws.pick(MOUTHS.len()),
    }
}

pub(crate) fn build(choice: Monster12Choice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let body = BODIES[choice.body % BODIES.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&SILHOUETTES[choice.silhouette % SILHOUETTES.len()]);
    canvas.paint(&TOPS[choice.top % TOPS.len()]);
    canvas.paint(&BROWS[choice.brow % BROWS.len()]);
    canvas.paint(&EYES[choice.eyes % EYES.len()]);
    canvas.paint(&MOUTHS[choice.mouth % MOUTHS.len()]);
    canvas.finish(Kind::Monster, background, |ink| match ink {
        // Holes in the body show the background, as in 8x8 monsters.
        Ink::Dark => background,
        Ink::White => WHITE,
        Ink::Pupil => PUPIL,
        Ink::Accent => accent,
        Ink::Body | Ink::Hair | Ink::Frame | Ink::Field2 | Ink::Metal => body,
    })
}
