---
name: gsichtl playground
description: A component datasheet for a crate of pixel-art avatar generators, read and tried on one page.
colors:
  band-red: "#c8102e"
  band-red-deep: "#a00c24"
  coated-paper: "#f7f6f2"
  sheet-white: "#fdfdfb"
  white: "#ffffff"
  ink: "#16181b"
  ink-soft: "#4a4e54"
  hairline: "#cfcdc5"
  code-tint: "#ecebe5"
  checker: "#e3e1da"
typography:
  part-number:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "2.75rem"
    fontWeight: 700
    lineHeight: 1
    letterSpacing: "-0.03em"
  seed:
    fontFamily: "ui-monospace, SF Mono, Menlo, Consolas, Liberation Mono, monospace"
    fontSize: "1.75rem"
    fontWeight: 400
    lineHeight: 1.5
  part-name:
    fontFamily: "ui-monospace, SF Mono, Menlo, Consolas, Liberation Mono, monospace"
    fontSize: "1.5rem"
    fontWeight: 400
    lineHeight: 1.5
  lede:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 400
    lineHeight: 1.5
  body:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "1rem"
    fontWeight: 400
    lineHeight: 1.5
  heading:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "1rem"
    fontWeight: 700
    lineHeight: 1.5
  plate-name:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 700
    lineHeight: 1.5
  label:
    fontFamily: "Archivo, Helvetica Neue, Arial, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 700
    lineHeight: 1.5
  measure:
    fontFamily: "ui-monospace, SF Mono, Menlo, Consolas, Liberation Mono, monospace"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
  code:
    fontFamily: "ui-monospace, SF Mono, Menlo, Consolas, Liberation Mono, monospace"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
rounded:
  none: "0px"
spacing:
  xs: "0.5rem"
  sm: "0.75rem"
  md: "1rem"
  lg: "1.5rem"
  xl: "2.5rem"
  2xl: "3.5rem"
components:
  band:
    backgroundColor: "{colors.band-red}"
    textColor: "{colors.white}"
    padding: "1.25rem 1.5rem 1.1rem"
  seed-field:
    backgroundColor: "{colors.sheet-white}"
    textColor: "{colors.ink}"
    typography: "{typography.seed}"
    rounded: "{rounded.none}"
    padding: "0.6rem 1.25rem"
  tile:
    backgroundColor: "{colors.sheet-white}"
    textColor: "{colors.ink}"
    rounded: "{rounded.none}"
    height: "7.5rem"
  tile-hover:
    backgroundColor: "{colors.white}"
  tile-plate:
    backgroundColor: "{colors.sheet-white}"
    textColor: "{colors.ink}"
    typography: "{typography.plate-name}"
    padding: "0.4rem 0.6rem 0.5rem"
  tile-plate-selected:
    backgroundColor: "{colors.band-red}"
    textColor: "{colors.white}"
    typography: "{typography.plate-name}"
    padding: "0.4rem 0.6rem 0.5rem"
  button-primary:
    backgroundColor: "{colors.band-red}"
    textColor: "{colors.white}"
    typography: "{typography.heading}"
    rounded: "{rounded.none}"
    padding: "0.9rem 1rem"
  button-primary-hover:
    backgroundColor: "{colors.band-red-deep}"
    textColor: "{colors.white}"
  button-copy:
    backgroundColor: "{colors.code-tint}"
    textColor: "{colors.band-red}"
    typography: "{typography.label}"
    rounded: "{rounded.none}"
    padding: "0.4rem 1rem"
    width: "6.5rem"
  button-copy-hover:
    backgroundColor: "{colors.sheet-white}"
    textColor: "{colors.band-red}"
  number-field:
    backgroundColor: "{colors.sheet-white}"
    textColor: "{colors.ink}"
    typography: "{typography.measure}"
    rounded: "{rounded.none}"
    padding: "0.25rem 0.4rem"
    width: "3.25rem"
  code-block:
    backgroundColor: "{colors.code-tint}"
    textColor: "{colors.ink}"
    typography: "{typography.code}"
    rounded: "{rounded.none}"
    padding: "0.75rem 1rem"
---

# Design System: gsichtl playground

## Overview

**Creative North Star: "The Component Datasheet"**

Every generator is a part with a number and a guaranteed output, and the page is its datasheet: read and tried in one place. The ground is coated paper in a warm off-white (#f7f6f2), the ink is near-black (#16181b), and one band of datasheet red carries the name and the version. Tables are ruled with hairlines, figures are tabular, the body face is a grotesk, and monospace appears only for code and measurements. The avatars are the only imagery; the system exists to hold them still and label them.

The density is that of a parts table. Thirteen tiles sit in two groups, each wearing a fixed name plate. The selected part opens as a ball-map: an avatar in a frame with lettered rows, numbered columns and a size equation. Nothing is rounded, nothing is shadowed, nothing moves. Depth is a rule weight, not a surface.

It refuses the crate-landing default: a hero headline, feature cards and a code block at the bottom. It also refuses to look decorated; an avatar is never put in a circle, a card or a gradient.

**Key Characteristics:**
- One red band, one ink, one paper. Red marks the band and what can be pressed, selected or is wrong.
- Hairline rules in two weights carry all structure; there are no shadows and no radii.
- Avatars are always drawn pixelated, on a fixed ramp of sizes, never smoothed.
- Monospace is for seeds, snippets and measurements. Everything else is Archivo.
- Every tile carries the same name plate: generator, grid, audience.

## Colors

A three-part palette: paper and sheet for surfaces, near-black for ink and heavy rules, one saturated red for the band and every control. Neutrals lean warm (hue near 95°) except the inks, which lean cool.

### Primary
- **Band Red** (#c8102e, oklch(53% 0.207 22)): the part-number band; the primary button; the selected tile's name plate and its inset outline; Copy button text; range and checkbox accents; the caret, text selection, focus ring; error text.
- **Band Red Deep** (#a00c24, oklch(45% 0.175 22)): hover state of the primary button only.

### Neutral
- **Coated Paper** (#f7f6f2): the page ground, and the scrollbar track.
- **Sheet White** (#fdfdfb): working surfaces that are written on or pressed: the seed row, tiles, number fields. The tiny lift from paper is the only surface step.
- **White** (#ffffff): text on red, and the hover fill of a tile.
- **Ink** (#16181b): body text, the 2px section rules, the plate and ruler outlines.
- **Ink Soft** (#4a4e54): secondary text: grid and audience on a plate, table headers, units, ruler labels, the status line.
- **Hairline** (#cfcdc5): 1px rules between rows, around tiles and around code blocks.
- **Code Tint** (#ecebe5): the code block ground.
- **Checker** (#e3e1da): the dark square of the transparency checker, paired with Sheet White; shown only behind a transparent render.

### Named Rules
**The Press Rule.** Red appears on the band, on what can be pressed or is selected, and on what is wrong. Body text, rules and decoration are never red. Test: remove the band and every red thing left must respond to a click or report an error.

**The Paper Rule.** Surfaces are paper or sheet. No dark mode, no tinted panels, no gradients except the transparency checker.

## Typography

**Display Font:** Archivo (with Helvetica Neue, Arial, sans-serif), self-hosted at 400 and 700, SIL OFL.
**Body Font:** Archivo, same stack.
**Label/Mono Font:** the system monospace stack (ui-monospace, SF Mono, Menlo, Consolas, Liberation Mono).

**Character:** A plain grotesk set in two weights reads like printed spec sheets; the system monospace keeps the technical voice honest without a second webfont. Tabular figures are on for the whole page so numbers align in columns and in the equation.

### Hierarchy
- **Part Number** (700, 2.75rem, line-height 1, -0.03em; 2.25rem under 30rem): `gsichtl` in the band, once. It is the page's only display size.
- **Seed** (mono 400, 1.75rem; 1.375rem under 30rem): the text typed into the seed field. The largest running type on the page after the name.
- **Part Name** (mono 400, 1.5rem): the selected generator's name in the detail heading.
- **Lede** (400, 1.125rem): the tagline, and the Seed label (700).
- **Body** (400, 1rem, 1.5): the base size; install line and checkbox text.
- **Heading** (700, 1rem): the Users, Groups, Install and Render options rules; the primary button label.
- **Plate Name** (700, 0.9375rem): a generator name on a tile. The kind it picked follows at weight 400.
- **Label** (700, 0.8125rem): table column headers (ink soft), the code block header and Copy button.
- **Measure** (mono, 0.875rem): the size equation, the units, ruler labels (0.6875rem, fitted to the cell), the snippet.

### Named Rules
**The Two Voices Rule.** Proportional type states and labels; monospace shows what the visitor types, what they copy and what is measured. A sentence is never set in monospace.

**The Two Weights Rule.** Weight 400 and 700 only. Hierarchy comes from size, weight and rule, never from colour or italics.

## Layout

A single column band, then a full-width seed row, then a two-column work area. The container is capped at 82rem with a 1.5rem side gap (1rem under 30rem). The band is a three-part row (name and version, tagline, links) aligned on the baseline. The seed row spans the container, ruled by 2px ink above and a hairline below, with the label in a ruled cell on the left.

The work area is 1.5fr to 1fr (the right column at least 22rem) with a 2.5rem gutter: the parts table on the left, the selected part on the right. Users are four tiles across, groups five. Under 66rem the detail moves below the tiles and the band stacks; under 30rem the tiles go three across. Install follows after 3.5rem, and a footer closes the page after 3rem.

Rhythm is a rem scale: 0.5, 0.75, 1, 1.5, 2.5, 3.5. Groups are 1.5rem apart; blocks inside the detail 1.25 to 1.5rem. Tiles sit on a 1px grid so the hairline between them is the only gutter.

### Named Rules
**The Ramp Rule.** An avatar is displayed at its native pixel size, or at a binary fraction of it (1, 1/2, 1/4, 1/8) when its column is narrower. Tiles show half size. Never at a free scale, and always `image-rendering: pixelated`. The ball-map rulers scale with it and thin out (every other label, then none) before they overlap.

## Elevation & Depth

Flat. There are no shadows. Depth is rule weight: a 2px ink rule opens each section, a 1px hairline separates rows and cells, and the selected tile adds a 2px inset red outline. The sheet-white surface against paper is the only tonal layering.

### Named Rules
**The Flat Rule.** No `box-shadow` except the inset selection outline. Do not lift, blur or stack surfaces.

**The Rule Weight Rule.** A section opens with 2px ink; everything inside it is separated with 1px hairline. A third weight does not exist.

## Shapes

Square. Every corner is 0: tiles, fields, buttons, code blocks, the plate. The ball-map frame is a 1px ink outline around the avatar, with a ruler on its top and left sides. The only non-rectilinear thing on the page is what the crate draws.

## Components

### Part-number Band
Full-width red, white text on one baseline row: `gsichtl` (Part Number), the version in bold mono, the tagline, then GitHub, docs.rs and crates.io links in bold white, underlined. Selection inverts to white on red. The version appears only after the crate has loaded.

### Seed Field
- **Shape:** a full-width row on Sheet White, 2px ink rule above, hairline below, no radius.
- **Style:** a bold label cell on the left, closed by a hairline; the input is borderless mono at 1.75rem.
- **Focus:** the 2px red outline, drawn inside (-2px offset).

### Tiles (the parts table)
- **Shape:** square cells on Sheet White, separated by 1px hairline outlines, in a 4-across (users) or 5-across (groups) grid.
- **Structure:** a 7.5rem frame centring the avatar, above a name plate: generator name in bold, picked kind in regular, and a second line `8 × 8 · users` in ink soft.
- **Hover:** fill becomes white.
- **Selected:** the plate turns Band Red with white text and a 2px inset red outline surrounds the tile. One tile is selected at all times.
- **Focus:** red 2px outline drawn inside.
- **Error:** the render error replaces the avatar, in bold red 0.8125rem.

### Ball-map (selected part)
The avatar sits in a 1px ink frame, over the checker only when transparent. A column ruler above is numbered, a row ruler to the left lettered, both in ink soft mono with a 1px ink edge. Below it a size equation `8 × 16 + 2 × 8 = 144` on the left and the bold `144 × 144 px` on the right.

### Render options table
A ruled datasheet table with the caption in a 2px ink rule. Columns: Parameter, Min, Value, Max. Each numeric row pairs a red range slider with a number field (Sheet White, 1px ink-soft border, mono, right-aligned, unit in ink soft). Invalid input turns red with a wavy underline. The checkbox row uses a red accent. Focus within a field draws the 2px red ring outside it.

### Code Block
- **Style:** Code Tint ground between two hairlines; a header row with a bold ink-soft label (`Rust`, `Cargo.toml`) and a Copy button separated by a hairline.
- **Body:** mono 0.875rem (1rem for the install line), wrapped, with 0.75rem 1rem padding.

### Footer
The colophon: a 2px ink rule across the container, then one row in 0.875rem ink soft, "gsichtl is made by Felix Itzenplitz." on the left and a bold ink, underlined link to https://itzenplitz.net/en/projects/ on the right. It wraps to two lines on narrow screens.

### Buttons
- **Primary (Download PNG):** full-width, Band Red with white bold 1rem text, 0.9rem by 1rem padding, no radius. Hover darkens to Band Red Deep. Focus ring turns ink, because red on red would disappear.
- **Copy:** a header cell, transparent, bold red text, 6.5rem minimum width so its "Copied" and "Copy failed" states do not shift the layout. Hover fills with Sheet White. Focus ring drawn inside.
- **Motion:** none. State changes are instant.

## Do's and Don'ts

### Do:
- **Do** draw every avatar with `image-rendering: pixelated` at native size or a binary fraction of it.
- **Do** give every generator tile the same plate: name, picked kind, grid, audience.
- **Do** use red only for the band, pressable or selected things, and errors; use Band Red Deep only for the hover of a filled red surface.
- **Do** open a section with a 2px ink rule and separate its rows with 1px hairlines.
- **Do** keep corners at 0 and surfaces flat.
- **Do** set seeds, snippets and measurements in monospace, and everything else in Archivo 400 or 700.
- **Do** keep a focus ring on every control: 2px red, ink on red fills, white on the band.

### Don't:
- **Don't** put an avatar in a circle, a card, a shadow or a gradient; the transparency checker is the only pattern.
- **Don't** smooth, blur or freely scale an avatar.
- **Don't** add imagery other than avatars the crate drew.
- **Don't** use `box-shadow` except the inset selection outline, or border-radius above 0.
- **Don't** use a third font weight, italics, a second accent colour or a dark mode.
- **Don't** set sentences in monospace or labels in red.
- **Don't** request fonts or assets from another origin; type is self-hosted.
