//! Realms: a small world for a group, a floating island, a planet or a
//! floating patch of sea, with a landmark, a sky detail and sometimes a
//! waterfall or a ring.
//!
//! Realms reuse the shared inks with their own colors: body (`#`) is the
//! biome's ground, the grass or the top; dark (`d`) its soil; pupil (`p`)
//! its stone; frame (`g`) its flora; field2 (`t`) its liquid. Hair (`h`) is
//! wood, white (`w`) is white, metal (`e`) is a warm glow, and accent (`a`)
//! is the realm's accent.

use crate::draws::Draws;
use crate::grid::{ACCENTS, BACKGROUNDS, Canvas, Ink, Layer, WHITE, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-03 realm v1";
pub(crate) const SIDE: u8 = 20;

const WOOD: Rgb = hex(0x6f4e37);
const GLOW: Rgb = hex(0xffd43b);

/// The colors a biome gives the ground, soil, stone, flora and liquid inks.
#[derive(Clone, Copy)]
struct Biome {
    ground: Rgb,
    soil: Rgb,
    stone: Rgb,
    flora: Rgb,
    liquid: Rgb,
}

const BIOMES: [Biome; 6] = [
    // meadow
    Biome {
        ground: hex(0x51cf66),
        soil: hex(0x8c5a2b),
        stone: hex(0x5c3d1e),
        flora: hex(0x2f9e44),
        liquid: hex(0x4dabf7),
    },
    // snow
    Biome {
        ground: hex(0xf1f3f5),
        soil: hex(0x868e96),
        stone: hex(0x495057),
        flora: hex(0x2b8a3e),
        liquid: hex(0x74c0fc),
    },
    // desert
    Biome {
        ground: hex(0xf4c97a),
        soil: hex(0xd9a05b),
        stone: hex(0xa0522d),
        flora: hex(0x5c940d),
        liquid: hex(0x22b8cf),
    },
    // lava
    Biome {
        ground: hex(0x5c5f66),
        soil: hex(0x7a3b2e),
        stone: hex(0x4a2520),
        flora: hex(0xc92a2a),
        liquid: hex(0xff6b00),
    },
    // mushroom
    Biome {
        ground: hex(0xbe4bdb),
        soil: hex(0x5f3dc4),
        stone: hex(0x3b2a6b),
        flora: hex(0xf03e3e),
        liquid: hex(0x63e6be),
    },
    // crystal
    Biome {
        ground: hex(0xc5f6fa),
        soil: hex(0x3b5bdb),
        stone: hex(0x2b3a8c),
        flora: hex(0x3bc9db),
        liquid: hex(0xe599f7),
    },
];

#[derive(Clone, Copy)]
enum Shape {
    Island,
    Planet,
    Floe,
}

/// Islands twice, so half of all realms are islands.
const SHAPES: [Shape; 4] = [Shape::Island, Shape::Island, Shape::Planet, Shape::Floe];

/// Landmarks per shape; every landmark uses the accent.
const LANDMARKS: usize = 5;
/// Sky details per shape, the first of them empty.
const DETAILS: usize = 4;

// A grassy top over a hanging clod of earth with roots.
const ISLAND: Layer = Layer::new(
    11,
    &[
        "...##############...",
        "..################..",
        "..dddddddddddddddd..",
        "...ddddddddddddddd..",
        "...hdddpdddddddd....",
        "....hddddddpddd.....",
        "....h.dddpdddd......",
        ".......dpddddh......",
        ".........ppd.h......",
    ],
    false,
);

// The island's clod with a pool of water instead of grass.
const FLOE: Layer = Layer::new(
    11,
    &[
        "..#ttttttttttttttt#.",
        "..#ttttttttttttttt#.",
        "..dddddddddddddddd..",
        "...ddddddddddddddd..",
        "...hdddpdddddddd....",
        "....hddddddpddd.....",
        "....h.dddpdddd......",
        ".......dpddddh......",
        ".........ppd.h......",
    ],
    false,
);

// A globe with seas and a shadowed lower right.
const PLANET: Layer = Layer::at(
    7,
    4,
    &[
        "....####....",
        "..##tt####..",
        ".##ttt#####.",
        ".#ttt####dd.",
        "#tt#####dddd",
        "##########dd",
        "####tt#####d",
        "###tttt###dd",
        ".##ttt###dd.",
        ".#######ddd.",
        "..####ddd...",
        "....dddd....",
    ],
    false,
);

// Spills from the top's right edge; island and floe.
const WATERFALL: Layer = Layer::at(
    11,
    14,
    &[
        "ttt...", "...tt.", "....t.", "....t.", "....t.", "....t.", "....t.", "....t.", "...w.w",
    ],
    false,
);

// Passes behind the planet in its middle row, in front below.
const RING: Layer = Layer::new(
    12,
    &[
        "ee................ee",
        ".eee............eee.",
        "...eeeeeeeeeeeeee...",
    ],
    false,
);

const GROUND_LANDMARKS: [Layer; LANDMARKS] = [
    // tree
    Layer::at(
        2,
        5,
        &[
            "...ggg....",
            "..ggggggg.",
            ".gggagggg.",
            "ggggggggag",
            "gaggggggg.",
            ".ggggggg..",
            "...ghhg...",
            "....hh....",
            "....hh....",
        ],
        false,
    ),
    // hut
    Layer::at(
        2,
        5,
        &[
            ".......p..",
            "...aaaap..",
            "..aaaaaaa.",
            ".aaaaaaaaa",
            "aaaaaaaaaa",
            ".hhhhhhhh.",
            ".heehhpph.",
            ".hhhhhpph.",
            ".hhhhhpph.",
        ],
        false,
    ),
    // tower
    Layer::at(
        1,
        6,
        &[
            "...h....", "...haa..", "...haaa.", "...h....", ".p.pp.p.", ".pppppp.", ".ppeppp.",
            ".pppppp.", ".pppppp.", ".pphhpp.",
        ],
        false,
    ),
    // windmill
    Layer::at(
        1,
        4,
        &[
            "w.......w...",
            "ww.....ww...",
            ".ww...ww....",
            "..wwhww.....",
            "...aah......",
            "..wwaaww....",
            ".ww.hh.ww...",
            "ww..hh..ww..",
            "w..hhhh..w..",
            "...hhhh.....",
        ],
        false,
    ),
    // tent
    Layer::at(
        4,
        4,
        &[
            "....h.......",
            "....haa.....",
            "....h.......",
            "...aaa......",
            "..aaaaa.....",
            ".aaapaaa.e..",
            "aaapppaa.ee.",
        ],
        false,
    ),
];

const SEA_LANDMARKS: [Layer; LANDMARKS] = [
    // ship
    Layer::at(
        1,
        4,
        &[
            ".....h......",
            ".....haa....",
            ".....h......",
            "....wh......",
            "...wwhw.....",
            "..wwwhww....",
            ".wwwwhwww...",
            ".....h......",
            "hhhhhhhhhhhh",
            ".hhahhahhhh.",
            "..hhhhhhhh..",
        ],
        false,
    ),
    // lighthouse
    Layer::at(
        1,
        6,
        &[
            "...pp...", "..peep..", "..wwww..", "...aa...", "...ww...", "...aa...", "..wwww..",
            "..aaaa..", "..wwww..", ".pppppp.", "pppppppp",
        ],
        false,
    ),
    // serpent
    Layer::at(
        4,
        3,
        &[
            "...........ggg",
            "..........gpgg",
            "...a..a....gaa",
            "..ggg.ggg..g..",
            ".gg.ggg.gg.g..",
            ".g...g...ggg..",
            "tg..tgt..tg...",
        ],
        false,
    ),
    // palm
    Layer::at(
        2,
        5,
        &[
            ".ggg.ggg..",
            "gg.ggg.gg.",
            "g..aha..g.",
            "....h.....",
            "....h.....",
            ".....h....",
            ".....h....",
            "...#####..",
            ".#########",
        ],
        false,
    ),
    // duck
    Layer::at(
        4,
        5,
        &[
            "...aaa....",
            "..aaaaa...",
            "..aapaaee.",
            "..aaaaa...",
            "a..aaa....",
            "aaaaaaaa..",
            "aaaaaaaa..",
            ".aaaaaa...",
        ],
        false,
    ),
];

const SPACE_LANDMARKS: [Layer; LANDMARKS] = [
    // baobab
    Layer::at(
        1,
        5,
        &[
            ".gggggggg.",
            "ggaggggagg",
            ".gg.hh.gg.",
            "....hh....",
            "...hhhh...",
            "..hhhhhh..",
        ],
        false,
    ),
    // rocket
    Layer::at(
        0,
        7,
        &[
            "..w...", ".wwa..", ".wew..", ".www..", "awwwa.", "a.e.a.", "..e...",
        ],
        false,
    ),
    // flag
    Layer::at(
        1,
        9,
        &["haaa", "haaa", "haa.", "h...", "h...", "h..."],
        false,
    ),
    // rose
    Layer::at(
        1,
        7,
        &[".www..", "w.aa.w", "w.aa.w", "w.g..w", "wgg..w", "w.g..w"],
        false,
    ),
    // dish
    Layer::at(
        1,
        6,
        &[
            "w......", ".ww..a.", ".wwwh..", "..wwww.", "...h...", "..hhh..",
        ],
        false,
    ),
];

const SKY_DETAILS: [Layer; DETAILS] = [
    // none
    Layer::new(0, &[], false),
    // cloud
    Layer::at(0, 14, &["..ww..", ".wwww.", "wwwwww"], false),
    // birds
    Layer::new(1, &["w.w....", ".w..w.w", ".....w."], false),
    // sun
    Layer::at(0, 16, &[".ee.", "eeee", "eeee", ".ee."], false),
];

const SPACE_DETAILS: [Layer; DETAILS] = [
    // none
    Layer::new(0, &[], false),
    // moon
    Layer::at(1, 1, &[".ww", "ww.", "ww.", ".ww"], false),
    // stars
    Layer::new(
        0,
        &[
            "..............e.....",
            ".e..................",
            ".................e..",
            "....................",
            "....................",
            "....................",
            "..e.................",
            "....................",
            "..................e.",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            ".e..................",
            ".................e..",
            "....................",
        ],
        false,
    ),
    // comet
    Layer::at(
        1,
        14,
        &[".....w", "....w.", "..ww..", "ee....", "ee...."],
        false,
    ),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[
    &[ISLAND, FLOE, PLANET],
    &[WATERFALL, RING],
    &GROUND_LANDMARKS,
    &SEA_LANDMARKS,
    &SPACE_LANDMARKS,
    &SKY_DETAILS,
    &SPACE_DETAILS,
];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RealmChoice {
    bg: usize,
    shape: usize,
    biome: usize,
    landmark: usize,
    accent: usize,
    /// A waterfall on an island or a floe, a ring around a planet.
    flourish: bool,
    detail: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> RealmChoice {
    RealmChoice {
        bg: draws.pick(BACKGROUNDS.len()),
        shape: draws.pick(SHAPES.len()),
        biome: draws.pick(BIOMES.len()),
        landmark: draws.pick(LANDMARKS),
        accent: draws.pick(ACCENTS.len()),
        flourish: draws.chance(50),
        detail: draws.pick(DETAILS),
    }
}

pub(crate) fn build(choice: RealmChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let biome = BIOMES[choice.biome % BIOMES.len()];
    let accent = ACCENTS[choice.accent % ACCENTS.len()];
    let (body, flourish, details, landmarks) = match SHAPES[choice.shape % SHAPES.len()] {
        Shape::Island => (&ISLAND, &WATERFALL, &SKY_DETAILS, &GROUND_LANDMARKS),
        Shape::Planet => (&PLANET, &RING, &SPACE_DETAILS, &SPACE_LANDMARKS),
        Shape::Floe => (&FLOE, &WATERFALL, &SKY_DETAILS, &SEA_LANDMARKS),
    };
    let mut canvas = Canvas::new(SIDE);
    canvas.paint(body);
    if choice.flourish {
        canvas.paint(flourish);
    }
    canvas.paint(&details[choice.detail % DETAILS]);
    // Last, so the landmark stands in front of the sky detail.
    canvas.paint(&landmarks[choice.landmark % LANDMARKS]);
    canvas.finish(Kind::Realm, background, |ink| match ink {
        Ink::Body => biome.ground,
        Ink::Dark => biome.soil,
        Ink::Pupil => biome.stone,
        Ink::Frame => biome.flora,
        Ink::Field2 => biome.liquid,
        Ink::Hair => WOOD,
        Ink::White => WHITE,
        Ink::Metal => GLOW,
        Ink::Accent => accent,
    })
}
