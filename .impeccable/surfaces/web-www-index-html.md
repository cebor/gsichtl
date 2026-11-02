---
version: 1
slug: "web-www-index-html"
primary_target: "web/www/index.html"
related_targets: ["web/www/style.css","web/www/app.js"]
---

## Scope

The whole one-page site for gsichtl: `web/www/index.html`, `web/www/style.css`, `web/www/app.js`. Visitor mode: Operate.

## Audience, job, constraints

See PRODUCT.md (Users, Product Purpose, Capabilities and Constraints). The job: type a seed, compare all 13 generators, pick one, tune cell/margin/transparency, copy the install line and the Rust call, download the PNG. Every avatar is drawn by the crate through `web/pkg` (wasm-bindgen glue); nothing pre-rendered. No third-party requests; any webfont self-hosted under `web/www/`. Avatars always `image-rendering: pixelated`. No imagery besides generated avatars.

## Functional spec

- Static (works without WASM): name `gsichtl`, the sentence "Deterministic pixel-art avatars from any seed.", install block, links GitHub / docs.rs / crates.io.
- Boot: `init()` from `./pkg/gsichtl_web.js`; on failure "The playground needs WebAssembly, which this browser could not load." in the playground area. After init: "Drawn by gsichtl {version}, compiled to WebAssembly." and the install line `gsichtl = { version = "{major}.{minor}", features = ["png"] }` with Copy.
- Seed field "Seed", initial `alice`; every input re-renders all 13 tiles and the detail; empty seed is valid.
- 13 tiles (button, aria-pressed) with sample-sheet defaults; `face`/`face12`/`icon` show the picked kind; selecting resets options to that generator's defaults; initial `face`.
- Detail: native-size canvas with current options, "{size} × {size} px", Cell 1–32, Margin 0–32, Transparent background; Rust snippet `let png = gsichtl::{name}({seed}).{to_png|to_png_transparent}({cell}, {margin})?;` with Copy; Download PNG `gsichtl-{name}-{seed}.png`.
- Copy: "Copied" 1.5 s, or "Copy failed" plus selecting the code.

## Direction contract

THESIS: Every generator is a part with a number and a guaranteed output; the page is its datasheet, read and tried in one place. It refuses the crate-landing default of hero headline, feature cards and a code block at the bottom.

OWN-WORLD: Coated paper white ground, near-black ink, one datasheet red band carrying name and version. Hairline-ruled tables, tabular figures, grotesk body, mono only for code and measurements. Red only on what can be pressed or is selected. Avatar scale on a fixed integer ramp. Every tile wears a fixed name plate: generator, grid, users or groups.

STORY: The visitor sees their seed drawn by the real crate thirteen ways, believes the pictures are exactly what their app will draw at this version, picks one, and leaves with the install line and the call.

FIRST VIEWPORT: Full-width red band on top: `gsichtl`, version, the sentence, links. Directly under it the Seed field at full width. Left two thirds: the 13 tiles as a dense parts table in two groups (users 8, groups 5). Right third: the selected avatar in a ball-map frame (rows lettered, columns numbered), size equation `side × cell + 2 × margin = px`, options, snippet with Copy, Download PNG. Install line closes the page.

FORM: Component datasheet, candidate 4 of 7 on the ordered list. seed:9e9c9781ad81d78f

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
