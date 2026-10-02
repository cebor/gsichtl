//! Crests: a shield, divided into two fields, with an emblem in metal.

use crate::draws::Draws;
use crate::grid::{BACKGROUNDS, Canvas, Ink, Layer, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-02 crest v1";
pub(crate) const SIDE: u8 = 10;

const FIELDS: [Rgb; 6] = [
    hex(0xc92a2a),
    hex(0x1864ab),
    hex(0x2b8a3e),
    hex(0x6741d9),
    hex(0xd9572b),
    hex(0x0c8599),
];
const METALS: [Rgb; 2] = [hex(0xffd43b), hex(0xf1f3f5)];

const FULL: &str = "##########";

const SHIELDS: [Layer; 3] = [
    // heater
    Layer::new(
        0,
        &[
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            ".########.",
            ".########.",
            "..######..",
            "....##....",
        ],
        false,
    ),
    // round
    Layer::new(
        0,
        &[
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            ".########.",
            "..######..",
            "...####...",
        ],
        false,
    ),
    // notched
    Layer::new(
        0,
        &[
            "#.######.#",
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            ".########.",
            "..######..",
            "...####...",
        ],
        false,
    ),
];

const PALE: &str = ".....ttttt";
const NONE: &str = "..........";
const FESS: &str = "tttttttttt";
const LEFT: &str = "ttttt.....";

const DIVISIONS: [Layer; 5] = [
    // plain
    Layer::new(0, &[], true),
    // per pale
    Layer::new(
        0,
        &[PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE],
        true,
    ),
    // per fess
    Layer::new(
        0,
        &[NONE, NONE, NONE, NONE, NONE, FESS, FESS, FESS, FESS, FESS],
        true,
    ),
    // quarterly
    Layer::new(
        0,
        &[LEFT, LEFT, LEFT, LEFT, LEFT, PALE, PALE, PALE, PALE, PALE],
        true,
    ),
    // bend
    Layer::new(
        0,
        &[
            "tt........",
            "ttt.......",
            ".ttt......",
            "..ttt.....",
            "...ttt....",
            "....ttt...",
            ".....ttt..",
            "......ttt.",
            ".......ttt",
            "........tt",
        ],
        true,
    ),
];

const EMBLEMS: [Layer; 8] = [
    // star
    Layer::at(
        2,
        2,
        &["..ee..", "..ee..", "eeeeee", ".eeee.", ".e..e.", "e....e"],
        true,
    ),
    // crown
    Layer::at(
        2,
        2,
        &["e.ee.e", "e.ee.e", "eeeeee", "eeeeee", "......", "eeeeee"],
        true,
    ),
    // heart
    Layer::at(
        2,
        2,
        &[".e..e.", "eeeeee", "eeeeee", ".eeee.", "..ee..", "......"],
        true,
    ),
    // bolt
    Layer::at(
        2,
        2,
        &["...ee.", "..ee..", ".eeee.", "..ee..", ".ee...", ".e...."],
        true,
    ),
    // lozenge
    Layer::at(
        2,
        2,
        &["..ee..", ".eeee.", "eeeeee", "eeeeee", ".eeee.", "..ee.."],
        true,
    ),
    // mug
    Layer::at(
        2,
        2,
        &["eeee..", "eeeeee", "eeee.e", "eeee.e", "eeeeee", "eeee.."],
        true,
    ),
    // cross
    Layer::at(
        2,
        2,
        &["..ee..", "..ee..", "eeeeee", "eeeeee", "..ee..", "..ee.."],
        true,
    ),
    // moon
    Layer::at(
        2,
        2,
        &["..eee.", ".ee...", "ee....", "ee....", ".ee...", "..eee."],
        true,
    ),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&SHIELDS, &DIVISIONS, &EMBLEMS];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CrestChoice {
    bg: usize,
    field: usize,
    field2: usize,
    metal: usize,
    shield: usize,
    division: usize,
    emblem: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> CrestChoice {
    let bg = draws.pick(BACKGROUNDS.len());
    let field = draws.pick(FIELDS.len());
    CrestChoice {
        bg,
        field,
        field2: draws.pick_other(FIELDS.len(), field),
        metal: draws.pick(METALS.len()),
        shield: draws.pick(SHIELDS.len()),
        division: draws.pick(DIVISIONS.len()),
        emblem: draws.pick(EMBLEMS.len()),
    }
}

pub(crate) fn build(choice: CrestChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let field = FIELDS[choice.field % FIELDS.len()];
    let field2 = FIELDS[choice.field2 % FIELDS.len()];
    let metal = METALS[choice.metal % METALS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&SHIELDS[choice.shield % SHIELDS.len()]);
    canvas.paint(&DIVISIONS[choice.division % DIVISIONS.len()]);
    canvas.paint(&EMBLEMS[choice.emblem % EMBLEMS.len()]);
    canvas.finish(Kind::Crest, background, |ink| match ink {
        Ink::Field2 => field2,
        Ink::Metal => metal,
        _ => field,
    })
}
