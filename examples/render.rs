//! Writes sample faces and crests as PNG files.
//!
//! `cargo run --example render --features png -- <out-dir> [count]`

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let Some(dir) = args.next().map(PathBuf::from) else {
        eprintln!("usage: render <out-dir> [count=32]");
        std::process::exit(2);
    };
    let count: u32 = match args.next() {
        Some(count) => count.parse()?,
        None => 32,
    };
    std::fs::create_dir_all(&dir)?;
    for n in 0..count {
        let face = gsichtl::face(format!("user-{n}").as_bytes());
        std::fs::write(dir.join(format!("face-{n:02}.png")), face.to_png(16, 8)?)?;
        let crest = gsichtl::crest(format!("community-{n}").as_bytes());
        std::fs::write(dir.join(format!("crest-{n:02}.png")), crest.to_png(16, 8)?)?;
    }
    Ok(())
}
