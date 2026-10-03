//! Retro: a game sprite for a group, a creature, a weapon, an item or a
//! piece of old tech, palette-swapped in neon colors, sometimes in front of
//! an explosion, a blood splatter or sparkles. Every sprite is an original
//! design in the style of classic games, not a copy of a real one.
//!
//! Retro sprites reuse the shared inks with their own colors: body (`#`) is
//! the neon base and dark (`d`) its shade; accent (`a`) is the second
//! neon's base; pupil (`p`) is the outline. White (`w`) is white, metal (`e`)
//! steel, hair (`h`) wood, frame (`g`) gold and field2 (`t`) blood.

use crate::draws::Draws;
use crate::grid::{BACKGROUNDS, Canvas, Ink, Layer, PUPIL, WHITE, hex};
use crate::{Avatar, Kind, Rgb};

pub(crate) const CONTEXT: &str = "gsichtl 2026-10-03 retro v1";
pub(crate) const SIDE: u8 = 20;

const STEEL: Rgb = hex(0xadb5bd);
const WOOD: Rgb = hex(0x9c6233);
const GOLD: Rgb = hex(0xfcc419);
const BLOOD: Rgb = hex(0xc92a2a);

/// A palette swap: the base color and its shade.
#[derive(Clone, Copy)]
struct Neon {
    base: Rgb,
    shade: Rgb,
}

const NEONS: [Neon; 8] = [
    // red
    Neon {
        base: hex(0xff6b6b),
        shade: hex(0xe03131),
    },
    // pink
    Neon {
        base: hex(0xf783ac),
        shade: hex(0xd6336c),
    },
    // orange
    Neon {
        base: hex(0xff922b),
        shade: hex(0xe8590c),
    },
    // yellow
    Neon {
        base: hex(0xffd43b),
        shade: hex(0xf59f00),
    },
    // green
    Neon {
        base: hex(0x69db7c),
        shade: hex(0x2f9e44),
    },
    // cyan
    Neon {
        base: hex(0x3bc9db),
        shade: hex(0x1098ad),
    },
    // blue
    Neon {
        base: hex(0x4dabf7),
        shade: hex(0x1c7ed6),
    },
    // purple
    Neon {
        base: hex(0x9775fa),
        shade: hex(0x6741d9),
    },
];

const MOTIFS: [Layer; 24] = [
    // ghost
    Layer::at(
        0,
        1,
        &[
            ".......ppppp.......",
            ".....pp#####pp.....",
            "....p#########p....",
            "...p###########p...",
            "..p#############p..",
            "..p##pp###pp####p..",
            "..p##pp###pp####p..",
            ".pp##pp###pp####pp.",
            "p#p#aa#####aa###p#p",
            "p########pp######p.",
            ".p######pppp####p..",
            "..p#####pppp###dp..",
            "..p######pp###ddp..",
            "..p###########ddp..",
            "..p##########dddp..",
            "...p#########dddp..",
            "....pp#####ddddp...",
            "......ppp#ddddp....",
            ".........pppddp....",
            "............pp.....",
        ],
        false,
    ),
    // invader
    Layer::at(
        1,
        1,
        &[
            ".....pp....pp.....",
            "....p##p..p##p....",
            "....p##pppp##p....",
            ".....pp####pp.....",
            "...pppp####pppp...",
            "..p############p..",
            ".pp############pp.",
            "p####aa####aa####p",
            "p####aa####aa####p",
            "p################p",
            "p################p",
            "p##dd########dd##p",
            "p##dd########dd##p",
            "p##ppddppppddpp##p",
            "p##ppddp..pddpp##p",
            ".pp.pddp..pddp.pp.",
            "....pddp..pddp....",
            ".....pp....pp.....",
        ],
        false,
    ),
    // skull
    Layer::at(
        1,
        2,
        &[
            "....pppppppp......",
            "..pp########pp....",
            ".p############p...",
            "p#w##w###w##w#ppp.",
            "p##############ddp",
            "pwwwwwwwwwwwwwwp#d",
            "pwwwwwwwwwwwwweppd",
            "pwwppppwwppppwep.p",
            "pwwpaapwwpaapwep..",
            "pwwppppwwppppwep..",
            "pwwwwwwppwwwwwep..",
            ".pwwwwwppwwwwep...",
            "..pwwwwwwwwwep....",
            "..pwpwpwpwpwep....",
            "..pwwwwwwwwwep....",
            "...peeeeeeeep.....",
            "....pppppppp......",
        ],
        false,
    ),
    // zombie
    Layer::at(
        0,
        2,
        &[
            ".....pp.p........",
            "...pp#dp#pppp....",
            "..p##########p...",
            ".p############p..",
            "p######dd######p.",
            "p##p######p#####p",
            "p#ppp####www####p",
            "p#pap####wpw####p",
            "p#ppp####www####p",
            "p#####t########dp",
            "p#####t#######ddp",
            "p###########ddddp",
            ".p#wpwpwpw####dp.",
            ".p#ttwttwtw###dp.",
            ".p###t######ddp..",
            "..p###t##ddddp...",
            "...p########p....",
            "....pppppppp.....",
        ],
        false,
    ),
    // slime
    Layer::at(
        3,
        1,
        &[
            "......pppppp.......",
            "....pp######pp.....",
            "...p##w#######p....",
            "..p#ww#########p...",
            ".p##w###########p..",
            ".p###############p.",
            "p####pp#####pp####p",
            "p####pp#####pp####p",
            "p#################p",
            "p###a#########a###p",
            "p#######pppp######p",
            "pd#######pp######dp",
            "pdpdd#########ddpdp",
            "pdppdddddddddddppdp",
            ".p..pppppdppppp..p.",
            ".........p.........",
        ],
        false,
    ),
    // mask
    Layer::new(
        0,
        &[
            ".......pppppp.......",
            ".....ppwwwwwepp.....",
            "....pwwwwwwwwwep....",
            "...pwwwwpwwpwwwep...",
            "..pwwaaawwwwaaawep..",
            "..pwwwawwwwwwawwep..",
            "pppwwwwwwwwwwwwweppp",
            "##wwpppwwwwwwpppwe##",
            "##wwpppwpwwpwpppwe##",
            "pppwwppwwwwwwppweppp",
            "..pwwwwwwwwwwwwwep..",
            "..pwpwwwwwwwwwwpep..",
            "..pwwwwaawwaawwwep..",
            "...pwwwwappawwwep...",
            "...pwwwwwwwwwwwep...",
            "....pwpwwwwwwpep....",
            ".....pwwpwwpwep.....",
            "......pwwwwwep......",
            ".......pppppp.......",
        ],
        false,
    ),
    // sword
    Layer::at(
        0,
        1,
        &[
            "...............ppp.",
            "..............ppppp",
            ".............ppwwpp",
            "............ppwwepp",
            "...........ppwwepp.",
            "..........ppwwepp..",
            ".........ppwwepp...",
            "........ppwwepp....",
            "...p...ppwwepp.....",
            "..ppp.ppwwepp......",
            ".ppgpppwwepp.......",
            ".ppggpwwepp........",
            "..ppggwepp.........",
            "...ppggpp..........",
            "..pp#pggpp.........",
            ".pp#dpppgpp........",
            "ppaapp.ppp.........",
            "ppaapp..p..........",
            ".pppp..............",
            "..pp...............",
        ],
        false,
    ),
    // blaster
    Layer::at(
        2,
        1,
        &[
            "...pppppppppp......",
            "..p##########ppp.p.",
            ".p#w##########eepep",
            "p############eeeeaa",
            "p############eeeeaa",
            "p##d#d#d#####eepepp",
            ".p##########dpp.p..",
            "..pdd######ddp.....",
            "..phhhppppppp......",
            "..phhhpppp.........",
            ".phhhppppp.........",
            ".phhhp.pp..........",
            "phhhp..............",
            "phhhp..............",
            "pdddp..............",
            ".ppp...............",
        ],
        false,
    ),
    // shells
    Layer::at(
        2,
        2,
        &[
            "..pp..............",
            ".p##p.............",
            "p####p..pp........",
            "p#w##p.p##p.......",
            "p#w##pp####p..pp..",
            "p#w##pp#w##p.p##p.",
            "paaaapp#w##pp####p",
            "p#w##pp#w##pp#w##p",
            "p####ppaaaapp#w##p",
            "pddddpp####pp#w##p",
            "pggggppddddppaaaap",
            "pgwggppggggppddddp",
            "pggggppgwggppggggp",
            "pddddppggggppgwggp",
            ".pppp.pddddppggggp",
            ".......pppp.pddddp",
            ".............pppp.",
        ],
        false,
    ),
    // grenade
    Layer::at(
        0,
        2,
        &[
            "...pp...........",
            "..paap.pppp.....",
            ".pappapeeeep....",
            ".pappaeeeeeeppp.",
            "..paapeeeeeeeeep",
            "...pppeeeepppeep",
            "...p########ppep",
            "..p##########pep",
            ".p#w##d##d##d#p.",
            "p##w##d##d##d##p",
            "p#dddddddddddd#p",
            "p##w##d##d##d##p",
            "p##w##d##d##d##p",
            "p#dddddddddddd#p",
            "p#####d##d##d##p",
            ".p####d##d##d#p.",
            "..p##########p..",
            "...ppddddddpp...",
            ".....pppppp.....",
        ],
        false,
    ),
    // crosshair
    Layer::new(
        0,
        &[
            ".........pp.........",
            "........p##p........",
            "......ppp##ppp......",
            "....pppp####pppp....",
            "...ppp##p##p##ppp...",
            "..pp##ppp##ppp##pp..",
            "..pp#p..p##p..p#pp..",
            ".pp#p...p##p...p#pp.",
            ".pp#ppp..pp..ppp#pp.",
            "p######ppaapp######p",
            "p######ppaapp######p",
            ".pp#ppp..pp..ppp#pp.",
            ".pp#p...p##p...p#pp.",
            "..pp#p..p##p..p#pp..",
            "..pp##ppp##ppp##pp..",
            "...ppp##p##p##ppp...",
            "....pppp####pppp....",
            "......ppp##ppp......",
            "........p##p........",
            ".........pp.........",
        ],
        false,
    ),
    // bomb
    Layer::at(
        0,
        2,
        &[
            "............pgpgp.",
            "...........pgpapp.",
            "...........ppagpgp",
            "..........phhpgpp.",
            ".........phpp.p...",
            "......ppphp.......",
            "....ppeeeepp......",
            "...p##eeee##p.....",
            "..p##########p....",
            ".p#w##########p...",
            "p#w############p..",
            "p#ww###########p..",
            "p#w###########dp..",
            "p#############dp..",
            "p############ddp..",
            ".p##########ddp...",
            "..p########ddp....",
            "...ppddddddpp.....",
            ".....pppppp.......",
        ],
        false,
    ),
    // mushroom
    Layer::at(
        1,
        1,
        &[
            "......pppppp......",
            "....pp######p.....",
            "...p##aa#####p....",
            "..p##aaaa##aa#p...",
            ".p###aaaa#aaaa#p..",
            ".p####aa##aaaa##p.",
            "p#########aa####dp",
            "p#aa############dp",
            "paaaa#####aa###ddp",
            "p#aa#####aaaa#dddp",
            ".pdddd####aaddddp.",
            "..ppwwwwwwwwwwpp..",
            "...pwwwwwwwwwep...",
            "...pwwwwwwwwwep...",
            "...pwwwwwwwwwep...",
            "....pwwwwwwwep....",
            ".....peeeeeep.....",
            "......pppppp......",
        ],
        false,
    ),
    // potion
    Layer::at(
        0,
        2,
        &[
            "......pppp......",
            ".....phhhhp.....",
            ".....phhhhp.....",
            "......peep......",
            "......peep......",
            "....ppeeeepp....",
            "...p########p...",
            "..p###a######p..",
            ".p#w##########p.",
            "p#ww#a#########p",
            "p#w############p",
            "p##############p",
            "p#######a######p",
            "p#############dp",
            ".p###########dp.",
            ".p##########ddp.",
            "..p########ddp..",
            "...ppddddddpp...",
            ".....pppppp.....",
        ],
        false,
    ),
    // chest
    Layer::at(
        2,
        2,
        &[
            "....pppppppp.....",
            "..pp########pp...",
            ".p############p..",
            "p#w##########d#p.",
            "p#############dp.",
            "pgggggggggggggggp",
            "pg####ggg####dgp.",
            "pddddgaaagdddddp.",
            "phhhhgapagdhhhhp.",
            "phhhhhgggddhhhhp.",
            "phhhhhhhhhhhhhhp.",
            "phdhdhhhhhhdhdhp.",
            "pgggggggggggggggp",
            "phhhhhhhhhhhhhhp.",
            "pddddddddddddddp.",
            ".pppppppppppppp..",
        ],
        false,
    ),
    // medkit
    Layer::at(
        2,
        1,
        &[
            "......pppppp......",
            ".....peeeeeep.....",
            ".ppppeppppppepppp.",
            "p################p",
            "p#w#############dp",
            "p#######ww######dp",
            "p#######ww######dp",
            "p####wwwwwwww###dp",
            "p####wwwwwwww###dp",
            "p#######ww######dp",
            "p#######ww######dp",
            "paaaaaaaaaaaaaaaap",
            "p###############dp",
            "pddddddddddddddddp",
            ".pppppppppppppppp.",
        ],
        false,
    ),
    // gem
    Layer::at(
        2,
        1,
        &[
            ".....pppppppp.....",
            "....p########p....",
            "...p#ww#####d#p...",
            "..p#ww###w###d#p..",
            ".p#w####www###d#p.",
            "p################p",
            ".p#w##########ddp.",
            "..p#w########ddp..",
            "...p########ddp...",
            "....p######ddp....",
            ".....p####ddp.....",
            "......p##ddp......",
            "..p....pddp.......",
            ".pap....pp....p...",
            "paaap........pap..",
            ".pap........paaap.",
            "..p..........pap..",
            "..............p...",
        ],
        false,
    ),
    // ufo
    Layer::new(
        4,
        &[
            ".......ppppp........",
            "......pwwwwwp.......",
            ".....pwawwwwwp......",
            "....pwaaawwwwep.....",
            "...ppwwawwwwweppp...",
            ".ppeeeeeeeeeeeeeepp.",
            "p##################p",
            "#w##g##g##g##g##g##d",
            "#ddddddddddddddddddd",
            "pddddddddddddddddddp",
            ".ppeeeeeeeeeeeeeepp.",
            "...ppaappaappaapp...",
            ".....pp..pp..pp.....",
        ],
        false,
    ),
    // gamepad
    Layer::new(
        4,
        &[
            "...pppppp....pppppp.",
            "..p######pppp######p",
            ".p#w######ee#######p",
            "p##################p",
            "p####p#########a###p",
            "p###ppp#ee#ee#a#a##p",
            "p####p#########a###p",
            "p##################p",
            "p#######dddddd####dp",
            "p######dppppppd###dp",
            ".p####dp......pdddp.",
            "..pddpp........ppp..",
            "...pp...............",
        ],
        false,
    ),
    // cassette
    Layer::at(
        2,
        1,
        &[
            ".pppppppppppppppp..",
            "p################p.",
            "p#wwwwwwwwwwwwww#dp",
            "p#aaaaaaaaaaaaaa#dp",
            "p#wwppppppppppww#dp",
            "p#wppppwwwwppppw#dp",
            "p#wwppppppppppww#dp",
            "p#aaaaaaaaaaaaaa#dp",
            "p#wwwwwwwwwwwwww#dp",
            "p################dp",
            "p###eeeeeeeeee###dp",
            "p##eepeeeeeepee##dp",
            "pdddddddddddddddddd",
            ".pppppppppppppppppp",
        ],
        false,
    ),
    // floppy
    Layer::at(
        0,
        1,
        &[
            ".ppppppppppppppp...",
            "p###############p..",
            "p###eeeeeeee##p##p.",
            "p###eeeeppee#####p.",
            "p###eeeeppee######p",
            "p###eeeeeeee######p",
            "p#################p",
            "p#################p",
            "p##wwwwwwwwwwwww##p",
            "p##waaaaaaaaaaaw##p",
            "p##wwwwwwwwwwwww##p",
            "p##wpppppppppppw##p",
            "p##wwwwwwwwwwwww##p",
            "p##wpppppppwpwww##p",
            "pp#wwwwwwwwwwwww##p",
            "pddddddddddddddddp.",
            ".pppppppppppppppp..",
        ],
        false,
    ),
    // joystick
    Layer::new(
        1,
        &[
            "...pppp.............",
            "..paaaap............",
            ".paaaaaap...........",
            ".pawaaaap...........",
            ".paaaaaap...........",
            "..paaaap............",
            "...peep.............",
            "...peep.............",
            "..ppeepp............",
            ".peeeeeepppppppppp..",
            ".p################p.",
            "p#w################p",
            "p############aa#ww#p",
            "p####dddd####aa#ww#p",
            "p#################dp",
            "p#################dp",
            "pdddddddddddddddddp.",
            ".ppppppppppppppppp..",
        ],
        false,
    ),
    // arcade
    Layer::at(
        0,
        1,
        &[
            "..paaaaaaaaaaaap..",
            "..pawaaawaaawaap..",
            "..paaaaaaaaaaaap..",
            ".p##############p.",
            ".p#pppppppppppp#p.",
            ".p#pppappppappp#p.",
            ".p#ppppappapppp#p.",
            ".p#pppppppppppp#p.",
            ".p#ppppppwppppp#p.",
            ".p#pppppwwwpppp#p.",
            "peeeeeeeeeeeeeeeep",
            "peepeeeweeaeeeeedp",
            ".p##############p.",
            ".p######ee######p.",
            ".p######aa######p.",
            ".p######ee######p.",
            ".p##############p.",
            ".pddddddddddddddp.",
            "..pppppppppppppp..",
        ],
        false,
    ),
    // ship
    Layer::new(
        0,
        &[
            "........pwwp........",
            ".......pweewp.......",
            ".......peeeep.......",
            "......paeaaeap......",
            "......paaaaaap......",
            "......p#aeea#p......",
            ".....p##eeee##p.....",
            "....p###eeee###p....",
            "...pd###e##e###dp...",
            "..pdd##e####e##ddp..",
            ".pddd##########dddp.",
            "pd#######ww#######dp",
            "pd##e##########e##dp",
            "d##ee##eeeeee##ee##d",
            "d#pppppppeeppppppp#d",
            "dp...pggppppggp...pd",
            "p....pggp..pggp....p",
            "......pap..pap......",
            ".......p....p.......",
        ],
        false,
    ),
];

const EFFECTS: [Layer; 4] = [
    // none
    Layer::new(0, &[], false),
    // burst
    Layer::new(
        1,
        &[
            ".........gg.........",
            "..g......gg......g..",
            "..gg....gggg....gg..",
            "...gg...gggg...gg...",
            "...gggggggggggggg...",
            "....gggggggggggg....",
            "....gggggggggggg....",
            "..gggggggggggggggg..",
            "gggggggggggggggggggg",
            "gggggggggggggggggggg",
            "..gggggggggggggggg..",
            "....gggggggggggg....",
            "....gggggggggggg....",
            "...gggggggggggggg...",
            "...gg...gggg...gg...",
            "..gg....gggg....gg..",
            "..g......gg......g..",
            ".........gg.........",
        ],
        false,
    ),
    // splatter
    Layer::new(
        1,
        &[
            ".tt..t...........t..",
            "tttt............ttt.",
            "ttttt..t.........t..",
            ".tttt...............",
            "..tt...............t",
            "t...................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "..................t.",
            ".t..............tttt",
            "...............ttttt",
            "..t..t........tttt..",
            "......t..t.....ttt..",
            "...............t....",
        ],
        false,
    ),
    // sparkles
    Layer::new(
        0,
        &[
            "..a.................",
            ".aaa.............g..",
            "..a.............ggg.",
            ".................g..",
            "....................",
            "...................a",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "....................",
            "a...................",
            "....................",
            "..g.................",
            ".ggg..............a.",
            "..g..............aaa",
            "..................a.",
        ],
        false,
    ),
];

#[cfg(test)]
pub(crate) const PARTS: &[&[Layer]] = &[&MOTIFS, &EFFECTS];

/// Indices into the palettes and part tables.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RetroChoice {
    bg: usize,
    motif: usize,
    neon: usize,
    /// Never `neon`, so the details always stand out.
    second: usize,
    effect: usize,
}

pub(crate) fn draw(draws: &mut Draws) -> RetroChoice {
    let bg = draws.pick(BACKGROUNDS.len());
    let motif = draws.pick(MOTIFS.len());
    let neon = draws.pick(NEONS.len());
    RetroChoice {
        bg,
        motif,
        neon,
        second: draws.pick_other(NEONS.len(), neon),
        effect: draws.pick(EFFECTS.len()),
    }
}

pub(crate) fn build(choice: RetroChoice) -> Avatar {
    let background = BACKGROUNDS[choice.bg % BACKGROUNDS.len()];
    let neon = NEONS[choice.neon % NEONS.len()];
    let second = NEONS[choice.second % NEONS.len()];
    let mut canvas = Canvas::new(SIDE);
    // First, so the motif and its outline stand in front of the effect.
    canvas.paint(&EFFECTS[choice.effect % EFFECTS.len()]);
    canvas.paint(&MOTIFS[choice.motif % MOTIFS.len()]);
    canvas.finish(Kind::Retro, background, |ink| match ink {
        Ink::Body => neon.base,
        Ink::Dark => neon.shade,
        Ink::Accent => second.base,
        Ink::Pupil => PUPIL,
        Ink::White => WHITE,
        Ink::Metal => STEEL,
        Ink::Hair => WOOD,
        Ink::Frame => GOLD,
        Ink::Field2 => BLOOD,
    })
}
