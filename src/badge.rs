//! Badges: a round disc, divided into two fields, with an emblem in metal and sometimes a metal rim.

use crate::crest::{EMBLEMS as CREST_EMBLEMS, FIELDS, METALS};
use crate::draws::Draws;
use crate::grid::{BACKGROUNDS, Canvas, Ink, Layer};
use crate::{Avatar, Kind};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-03 badge v1";
pub(crate) const SIDE: u8 = 12;

const FULL: &str = "############";

const SHAPES: [Layer; 3] = [
    // disc
    Layer::new(
        0,
        &[
            "....####....",
            "..########..",
            ".##########.",
            ".##########.",
            FULL,
            FULL,
            FULL,
            FULL,
            ".##########.",
            ".##########.",
            "..########..",
            "....####....",
        ],
        false,
    ),
    // octagon
    Layer::new(
        0,
        &[
            "...######...",
            "..########..",
            ".##########.",
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            FULL,
            ".##########.",
            "..########..",
            "...######...",
        ],
        false,
    ),
    // cog
    Layer::new(
        0,
        &[
            "...##..##...",
            "..########..",
            ".##########.",
            FULL,
            FULL,
            ".##########.",
            ".##########.",
            FULL,
            FULL,
            ".##########.",
            "..########..",
            "...##..##...",
        ],
        false,
    ),
];

const RIMS: [Layer; 3] = [
    // disc
    Layer::new(
        0,
        &[
            "....eeee....",
            "..ee....ee..",
            ".e........e.",
            ".e........e.",
            "e..........e",
            "e..........e",
            "e..........e",
            "e..........e",
            ".e........e.",
            ".e........e.",
            "..ee....ee..",
            "....eeee....",
        ],
        true,
    ),
    // octagon
    Layer::new(
        0,
        &[
            "...eeeeee...",
            "..e......e..",
            ".e........e.",
            "e..........e",
            "e..........e",
            "e..........e",
            "e..........e",
            "e..........e",
            "e..........e",
            ".e........e.",
            "..e......e..",
            "...eeeeee...",
        ],
        true,
    ),
    // cog
    Layer::new(
        0,
        &[
            "...ee..ee...",
            "..eeeeeeee..",
            ".e........e.",
            "ee........ee",
            "ee........ee",
            ".e........e.",
            ".e........e.",
            "ee........ee",
            "ee........ee",
            ".e........e.",
            "..eeeeeeee..",
            "...ee..ee...",
        ],
        true,
    ),
];

const PALE: &str = "......tttttt";
const NONE: &str = "............";
const FESS: &str = "tttttttttttt";
const LEFT: &str = "tttttt......";

const DIVISIONS: [Layer; 5] = [
    // plain
    Layer::new(0, &[], true),
    // per pale
    Layer::new(
        0,
        &[
            PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE, PALE,
        ],
        true,
    ),
    // per fess
    Layer::new(
        0,
        &[
            NONE, NONE, NONE, NONE, NONE, NONE, FESS, FESS, FESS, FESS, FESS, FESS,
        ],
        true,
    ),
    // quarterly
    Layer::new(
        0,
        &[
            LEFT, LEFT, LEFT, LEFT, LEFT, LEFT, PALE, PALE, PALE, PALE, PALE, PALE,
        ],
        true,
    ),
    // bend
    Layer::new(
        0,
        &[
            "tt..........",
            "ttt.........",
            ".ttt........",
            "..ttt.......",
            "...ttt......",
            "....ttt.....",
            ".....ttt....",
            "......ttt...",
            ".......ttt..",
            "........ttt.",
            ".........ttt",
            "..........tt",
        ],
        true,
    ),
];

// The crest emblems, centered on the larger grid.
const EMBLEMS: [Layer; 8] = {
    let mut layers = CREST_EMBLEMS;
    let mut i = 0;
    while i < layers.len() {
        layers[i].top = 3;
        layers[i].left = 3;
        i += 1;
    }
    layers
};

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&SHAPES, &RIMS, &DIVISIONS, &EMBLEMS];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BadgeChoice {
    bg: usize,
    field: usize,
    field2: usize,
    metal: usize,
    shape: usize,
    rim: bool,
    division: usize,
    emblem: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> BadgeChoice {
    let bg = draws.pick(BACKGROUNDS.len());
    let field = draws.pick(FIELDS.len());
    BadgeChoice {
        bg,
        field,
        field2: draws.pick_other(FIELDS.len(), field),
        metal: draws.pick(METALS.len()),
        shape: draws.pick(SHAPES.len()),
        rim: draws.chance(50),
        division: draws.pick(DIVISIONS.len()),
        emblem: draws.pick(EMBLEMS.len()),
    }
}

pub(crate) fn build(choice: BadgeChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let field = FIELDS[choice.field % FIELDS.len()];
    let field2 = FIELDS[choice.field2 % FIELDS.len()];
    let metal = METALS[choice.metal % METALS.len()];
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(&SHAPES[choice.shape % SHAPES.len()]);
    canvas.paint(&DIVISIONS[choice.division % DIVISIONS.len()]);
    if choice.rim {
        // After the division, so the rim stays metal.
        canvas.paint(&RIMS[choice.shape % RIMS.len()]);
    }
    canvas.paint(&EMBLEMS[choice.emblem % EMBLEMS.len()]);
    canvas.finish(Kind::Badge, background, |ink| match ink {
        Ink::Field2 => field2,
        Ink::Metal => metal,
        _ => field,
    })
}
