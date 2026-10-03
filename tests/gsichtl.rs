use std::collections::HashSet;

use gsichtl::{Avatar, Kind};

fn fingerprint(avatar: &Avatar) -> String {
    blake3::hash(&avatar.to_rgba(1, 0).pixels)
        .to_hex()
        .to_string()
}

/// Every client has to draw the same picture for the same id; a change here
/// is a breaking change of the crate (see the crate docs on stability).
#[test]
fn seeds_keep_their_pictures() {
    let zero = [0u8; 32];
    let got = [
        fingerprint(&gsichtl::face(b"alice")),
        fingerprint(&gsichtl::monster(b"alice")),
        fingerprint(&gsichtl::nerd(b"alice")),
        fingerprint(&gsichtl::crest(b"alice")),
        fingerprint(&gsichtl::badge(b"alice")),
        fingerprint(&gsichtl::face(&zero)),
        fingerprint(&gsichtl::monster(&zero)),
        fingerprint(&gsichtl::nerd(&zero)),
        fingerprint(&gsichtl::crest(&zero)),
        fingerprint(&gsichtl::badge(&zero)),
    ];
    let want = [
        "b79f342b2e7442884798392a11322461b9a4cdb70cce51d9cc24c06bcddff32a",
        "f8e326f0fe6bc04b8147df61f7634f411c63e9c01d29b9f2f8cd12532610c1bc",
        "157756544a686e50e5ad217928b15346484e143f7e8e94e59e311d58e312867b",
        "7bf1be0e54721a0bb29df909393911c1efea3c8214e7eee0937f8cab2eb2cffe",
        "956b990ab687f4e2ded29257750684fcf231b02a62129ed66b6545bbe44acba9",
        "0bc86f28b5b9909abb0f000a871953b2cd68e3f3d44ff1b4ee8a3949b7b77cde",
        "e444dc102577ec0713cf8bf1e2c787e35ad3e323964f05cf7c86a4db9dd52af5",
        "16f5370130d7c99666b42fbd9ed28f50f9a116ba87651b277a4baa39c31f0e1f",
        "3e834c8851233b181d6f0e979f3bc9b0fc9c4d1cc73d2a10b2375380bd9d4243",
        "b08567a921a7b9a0b678ad211f9f2bab81d701ba0e63efa168131a4caa3f6988",
    ];
    assert_eq!(got, want);
}

fn distinct(generate: fn(&[u8]) -> Avatar) -> usize {
    (0u32..1000)
        .map(|n| generate(&n.to_le_bytes()))
        .collect::<HashSet<_>>()
        .len()
}

#[test]
fn distinct_seeds_look_distinct() {
    assert!(distinct(gsichtl::face) >= 990);
    assert!(distinct(gsichtl::monster) >= 990);
    assert!(distinct(gsichtl::nerd) >= 990);
    // Only 28,800 crests exist, so about 17 collisions in 1000 are expected.
    assert!(distinct(gsichtl::crest) >= 950);
    // Only 57,600 badges exist, so a few collisions in 1000 are expected.
    assert!(distinct(gsichtl::badge) >= 950);
}

#[test]
fn faces_mix_monsters_and_nerds() {
    let kinds: Vec<Kind> = (0u32..1000)
        .map(|n| gsichtl::face(&n.to_le_bytes()).kind())
        .collect();
    let monsters = kinds.iter().filter(|&&k| k == Kind::Monster).count();
    assert!(kinds.iter().all(|&k| k == Kind::Monster || k == Kind::Nerd));
    assert!((400..=600).contains(&monsters), "{monsters} monsters");
}

#[test]
fn rgba_places_cells_and_margin() {
    let crest = gsichtl::crest(b"x");
    let image = crest.to_rgba(3, 2);
    assert_eq!((image.width, image.height), (34, 34));
    assert_eq!(image.pixels.len(), 34 * 34 * 4);
    let pixel = |x: usize, y: usize| {
        let i = (y * 34 + x) * 4;
        [image.pixels[i], image.pixels[i + 1], image.pixels[i + 2]]
    };
    assert_eq!(pixel(0, 0), crest.background());
    for y in 0..crest.side() {
        for x in 0..crest.side() {
            let want = crest.cell(x, y).unwrap_or(crest.background());
            assert_eq!(
                pixel(2 + 3 * x as usize + 1, 2 + 3 * y as usize + 1),
                want,
                "cell ({x}, {y})"
            );
        }
    }
    assert!(image.pixels.chunks(4).all(|p| p[3] == 255));
}

#[test]
fn transparent_rgba_clears_only_the_background() {
    let crest = gsichtl::crest(b"x");
    let image = crest.to_rgba_transparent(3, 2);
    assert_eq!((image.width, image.height), (34, 34));
    let pixel = |x: usize, y: usize| {
        let i = (y * 34 + x) * 4;
        [
            image.pixels[i],
            image.pixels[i + 1],
            image.pixels[i + 2],
            image.pixels[i + 3],
        ]
    };
    assert_eq!(pixel(0, 0), [0, 0, 0, 0]);
    let mut empty = 0;
    for y in 0..crest.side() {
        for x in 0..crest.side() {
            let want = match crest.cell(x, y) {
                Some([r, g, b]) => [r, g, b, 255],
                None => {
                    empty += 1;
                    [0, 0, 0, 0]
                }
            };
            assert_eq!(
                pixel(2 + 3 * x as usize + 1, 2 + 3 * y as usize + 1),
                want,
                "cell ({x}, {y})"
            );
        }
    }
    assert!(empty > 0);
}
