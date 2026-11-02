//! Hands gsichtl's version to the page, which names it beside the install line.

fn main() {
    println!("cargo::rerun-if-changed=../Cargo.toml");
    let manifest = std::fs::read_to_string("../Cargo.toml").expect("read ../Cargo.toml");
    let version = manifest
        .lines()
        .skip_while(|line| *line != "[package]")
        .find_map(|line| line.strip_prefix("version = \"")?.strip_suffix('"'))
        .expect("../Cargo.toml has a version in [package]");
    println!("cargo::rustc-env=GSICHTL_VERSION={version}");
}
