# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Static HTML, CSS and JavaScript, no framework and no npm. The generators run as the real `gsichtl` crate compiled to WebAssembly through a small wasm-bindgen glue crate in `web/` (`publish = false`), so the published crate keeps its single dependency. Built with `cargo` and `wasm-bindgen-cli`; deployed to GitHub Pages (https://cebor.github.io/gsichtl/) by `.github/workflows/pages.yml` on every release tag `v*` and by manual dispatch from `main`.

## Users

Rust developers evaluating gsichtl, arriving from crates.io, GitHub or docs.rs. They want to see what their own user and group ids turn into across the generators, pick the generator that fits their app, and copy the code to use it.

## Product Purpose

gsichtl draws deterministic pixel-art avatars from any seed: the same seed always gives the same picture, so every client draws it locally and nothing has to be stored or sent. The site lets a developer try every generator with their own seeds before adding the dependency. Success: they type their ids, find the generator they want, and leave with the install line and a working Rust call.

## Positioning

The page does not imitate the crate: every picture on it is drawn by the crate itself, compiled to WebAssembly, at the version shown on the page. What a seed shows here is what the crate draws in their app. *Gsichtl* is Bavarian for "little face".

## Operating Context

Visitors compare the site with the README's sample sheets (`docs/faces.png`, `docs/robots.png`, `docs/faces12.png`, `docs/groups.png`) and the API on docs.rs. Seeds are typically public ids; the sample sheets use `user-N` for user avatars and `community-N` for group avatars.

## Capabilities and Constraints

- 13 generators. For users: `face` (monster or nerd, picked by the seed), `monster`, `nerd`, `robot` at 8x8; `face12`, `monster12`, `nerd12`, `robot12` at 12x12. For groups: `crest` 10x10, `badge` 12x12, `realm` 20x20, `retro` 20x20, `icon` (realm or retro, picked by the seed).
- Rendering takes `cell_px` and `margin_px` (each a `u8`); `to_rgba`/`to_png` are opaque, `to_rgba_transparent`/`to_png_transparent` leave background and margin transparent. Realms, retro sprites and icons are made to stand free on a transparent background.
- The same seed gives the same picture within a `0.x` minor version; pictures are a contract.
- The crate does no I/O, forbids unsafe code and has one dependency (`blake3`), plus `png` behind an optional feature. The site must not add dependencies to the published crate.
- MIT licensed.

## Brand Commitments

Name `gsichtl`, lowercase. Voice: plain, factual, short sentences, as in README.md. The generated avatars are the only imagery.

## Evidence on Hand

The README's sample sheets in `docs/` and the per-generator variant counts in README.md. No user counts, download numbers, testimonials or adopters exist; never invent them.

## Product Principles

1. The crate draws every avatar on the page; nothing is reimplemented or pre-rendered.
2. Trying comes before reading: a seed field and the pictures are the page.
3. Determinism is visible: the same seed shows the same picture, and the version that draws it is named.
4. An avatar is a decoration, never an identity check.
