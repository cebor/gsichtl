//! Monsters: a body silhouette with something on top, brows, eyes and a mouth.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-02 monster v1";
pub(crate) const SIDE: u8 = 8;

const BODIES: [Rgb; 10] = [
    hex(0x7c5cff),
    hex(0xff6b6b),
    hex(0x4ecdc4),
    hex(0xffa94d),
    hex(0x69db7c),
    hex(0x4dabf7),
    hex(0xf783ac),
    hex(0xd9572b),
    hex(0xa9e34b),
    hex(0xe599f7),
];

const SILHOUETTES: [Layer; 3] = [
    // square
    Layer::new(
        2,
        &[
            ".######.", "########", "########", "########", "########", ".######.",
        ],
        false,
    ),
    // round
    Layer::new(
        2,
        &[
            "..####..", ".######.", "########", "########", ".######.", "..####..",
        ],
        false,
    ),
    // bell
    Layer::new(
        2,
        &[
            ".######.", ".######.", "########", "########", "########", "########",
        ],
        false,
    ),
];

const TOPS: [Layer; 6] = [
    // antennae
    Layer::new(0, &["..#..#..", "..#..#.."], false),
    // horns
    Layer::new(0, &["a......a", ".a....a."], false),
    // ears
    Layer::new(0, &[".#....#.", ".##..##."], false),
    // bobble
    Layer::new(0, &["...aa...", "...##..."], false),
    // spikes
    Layer::new(0, &["........", ".#.##.#."], false),
    // none
    Layer::new(0, &["........", "........"], false),
];

const BROWS: [Layer; 3] = [
    // slot
    Layer::new(3, &["..dddd.."], true),
    // none
    Layer::new(3, &["........"], true),
    // brows
    Layer::new(3, &[".dd..dd."], true),
];

const EYES: [Layer; 5] = [
    // looking right
    Layer::new(4, &["#ww##ww#", "#wp##wp#"], true),
    // looking left
    Layer::new(4, &["#ww##ww#", "#pw##pw#"], true),
    // looking up
    Layer::new(4, &["#pw##pw#", "#ww##ww#"], true),
    // cyclops
    Layer::new(4, &["##wwww##", "##wppw##"], true),
    // sleepy
    Layer::new(4, &["#dd##dd#", "#wp##wp#"], true),
];

const MOUTHS: [Layer; 5] = [
    // teeth
    Layer::new(6, &["########", ".#a#a#a."], true),
    // smile
    Layer::new(6, &["##dddd##"], true),
    // fangs
    Layer::new(6, &["##dddd##", "..a..a.."], true),
    // small
    Layer::new(6, &["###dd###"], true),
    // none
    Layer::new(6, &[], true),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&SILHOUETTES, &TOPS, &BROWS, &EYES, &MOUTHS];

/// Indices into the palettes and part tables; all zero is the reference bot.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MonsterChoice {
    bg: usize,
    body: usize,
    accent: usize,
    top: usize,
    silhouette: usize,
    brow: usize,
    eyes: usize,
    mouth: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> MonsterChoice {
    MonsterChoice {
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

pub(crate) fn build(choice: MonsterChoice) -> Avatar {
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
        // Holes in the body show the background, as in the reference bot.
        Ink::Dark => background,
        Ink::White => WHITE,
        Ink::Pupil => PUPIL,
        Ink::Accent => accent,
        Ink::Body | Ink::Hair | Ink::Frame | Ink::Field2 | Ink::Metal => body,
    })
}

#[cfg(test)]
mod tests {
    use super::{MonsterChoice, build};
    use crate::grid::hex;

    #[test]
    fn reference_bot() {
        let bot = build(MonsterChoice::default());
        let background = hex(0x1a1530);
        assert_eq!(bot.background(), background);
        assert_eq!(bot.side(), 8);
        let grid = [
            "..V..V..", "..V..V..", ".VVVVVV.", "VV....VV", "VWWVVWWV", "VWPVVWPV", "VVVVVVVV",
            ".VYVYVY.",
        ];
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                let want = match c {
                    b'V' => Some(hex(0x7c5cff)),
                    b'W' => Some(hex(0xffffff)),
                    b'P' => Some(hex(0x11151c)),
                    b'Y' => Some(hex(0xffd166)),
                    _ => None,
                };
                let got = bot.cell(x as u8, y as u8);
                // The brow slot is painted in the background color.
                let got = got.filter(|&c| !(c == background && want.is_none()));
                assert_eq!(got, want, "cell ({x}, {y})");
            }
        }
    }
}
