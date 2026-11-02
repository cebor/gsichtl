import init, { render, version } from "./pkg/gsichtl_web.js";

// Display order. The defaults are the README sample-sheet renders from examples/render.rs.
const GENERATORS = [
  { name: "face", side: 8, group: "users", cell: 16, margin: 8, transparent: false, picks: true },
  { name: "monster", side: 8, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "nerd", side: 8, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "robot", side: 8, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "face12", side: 12, group: "users", cell: 16, margin: 8, transparent: false, picks: true },
  { name: "monster12", side: 12, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "nerd12", side: 12, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "robot12", side: 12, group: "users", cell: 16, margin: 8, transparent: false },
  { name: "crest", side: 10, group: "groups", cell: 18, margin: 14, transparent: false },
  { name: "badge", side: 12, group: "groups", cell: 16, margin: 8, transparent: false },
  { name: "realm", side: 20, group: "groups", cell: 10, margin: 4, transparent: true },
  { name: "retro", side: 20, group: "groups", cell: 10, margin: 4, transparent: true },
  { name: "icon", side: 20, group: "groups", cell: 10, margin: 4, transparent: true, picks: true },
];

// Ball-map row letters as on a BGA package: I, O, Q, S, X and Z are skipped.
const ROWS = "ABCDEFGHJKLMNPRTUVWY";
// Tiles show their default render at half size: every default cell and margin is even,
// so each avatar cell stays a whole number of CSS pixels.
const TILE_SCALE = 0.5;
// The detail shrinks only along this ramp when it does not fit its column.
const DETAIL_SCALES = [1, 0.5, 0.25, 0.125];
// Below this many CSS pixels per cell the ball-map labels would collide; two-digit column
// numbers need more and then only every other one is shown.
const MIN_LABEL_PX = 9;
const MIN_TWO_DIGIT_PX = 15;

const $ = (id) => document.getElementById(id);
const status = $("status");

const state = {
  seed: "",
  selected: GENERATORS[0],
  cell: GENERATORS[0].cell,
  margin: GENERATORS[0].margin,
  transparent: GENERATORS[0].transparent,
};

try {
  await init();
} catch (error) {
  status.textContent = "The playground needs WebAssembly, which this browser could not load.";
  status.classList.add("is-error");
  throw error;
}

const gsichtlVersion = version();
const [major, minor] = gsichtlVersion.split(".");
status.textContent = `Drawn by gsichtl ${gsichtlVersion}, compiled to WebAssembly.`;
$("version").textContent = gsichtlVersion;
$("version").hidden = false;
$("install-label").textContent = "Cargo.toml";
$("install").textContent = `gsichtl = { version = "${major}.${minor}", features = ["png"] }`;

const seedInput = $("seed");
// Matches the breakpoint in style.css where the detail moves below the tiles.
const oneColumn = matchMedia("(max-width: 66rem)");
const tiles = new Map();

for (const generator of GENERATORS) {
  const button = document.createElement("button");
  button.type = "button";
  button.className = "tile";
  button.setAttribute("aria-pressed", "false");
  const frame = document.createElement("span");
  frame.className = "tile-frame";
  const canvas = document.createElement("canvas");
  const error = document.createElement("span");
  error.className = "render-error";
  error.hidden = true;
  frame.append(canvas, error);
  const plate = document.createElement("span");
  plate.className = "tile-plate";
  const name = document.createElement("code");
  name.className = "tile-name";
  name.textContent = generator.name;
  const kind = document.createElement("span");
  kind.className = "tile-kind";
  name.append(kind);
  const meta = document.createElement("span");
  meta.className = "tile-meta";
  meta.textContent = `${generator.side} × ${generator.side} · ${generator.group}`;
  plate.append(name, meta);
  button.append(frame, plate);
  button.addEventListener("click", () => {
    select(generator);
    // In one column the detail sits below the tiles: bring it to where the user looks.
    if (oneColumn.matches) $("detail-name").closest(".detail").scrollIntoView({ block: "start" });
  });
  $(`tiles-${generator.group}`).append(button);
  tiles.set(generator.name, { button, canvas, error, kind });
}

/** Draws `generator` for the current seed onto `canvas`; returns the kind it drew, or null. */
function draw(canvas, errorBox, generator, cell, margin, transparent, scale) {
  let picture;
  try {
    picture = render(generator.name, state.seed, cell, margin, transparent);
  } catch (error) {
    canvas.hidden = true;
    errorBox.hidden = false;
    errorBox.textContent = error instanceof Error ? error.message : String(error);
    return null;
  }
  canvas.hidden = false;
  errorBox.hidden = true;
  const kind = picture.kind;
  const size = picture.size;
  const rgba = picture.into_rgba();
  canvas.width = size;
  canvas.height = size;
  canvas.style.width = `${size * scale}px`;
  canvas.style.height = `${size * scale}px`;
  canvas
    .getContext("2d")
    .putImageData(new ImageData(new Uint8ClampedArray(rgba.buffer, rgba.byteOffset, rgba.length), size, size), 0, 0);
  return { kind, size };
}

function renderTiles() {
  for (const generator of GENERATORS) {
    const tile = tiles.get(generator.name);
    const drawn = draw(tile.canvas, tile.error, generator, generator.cell, generator.margin, generator.transparent, TILE_SCALE);
    tile.kind.textContent = generator.picks && drawn ? ` · ${drawn.kind}` : "";
    tile.button.setAttribute("aria-label", `${generator.name} for seed ${state.seed}`);
  }
}

const ballmap = $("ballmap");
const detailCanvas = $("detail-canvas");
let detailSize = 0;

function renderDetail() {
  const generator = state.selected;
  const drawn = draw(detailCanvas, $("detail-error"), generator, state.cell, state.margin, state.transparent, 1);
  $("detail-name").textContent = generator.name;
  $("detail-kind").textContent = generator.picks && drawn ? ` · ${drawn.kind}` : "";
  detailSize = drawn ? drawn.size : 0;
  $("equation").textContent = `${generator.side} × ${state.cell} + 2 × ${state.margin} = ${detailSize}`;
  $("size-label").textContent = `${detailSize} × ${detailSize} px`;
  ballmap.classList.toggle("is-transparent", state.transparent);

  const cols = $("ruler-cols");
  const rows = $("ruler-rows");
  if (cols.childElementCount !== generator.side) {
    cols.replaceChildren(...Array.from({ length: generator.side }, (_, i) => label(String(i + 1))));
    rows.replaceChildren(...Array.from({ length: generator.side }, (_, i) => label(ROWS[i])));
  }
  fitDetail();

  // <wbr> lets a narrow column break the call at its joints; the copied text has none.
  const method = state.transparent ? "to_png_transparent" : "to_png";
  $("snippet").replaceChildren(
    "let png = gsichtl::",
    document.createElement("wbr"),
    `${generator.name}(${rustSeed(state.seed)})`,
    document.createElement("wbr"),
    `.${method}(${state.cell}, ${state.margin})?;`,
  );
}

function label(text) {
  const span = document.createElement("span");
  span.textContent = text;
  return span;
}

/** Scales the detail down along DETAIL_SCALES until it fits its column, and sizes the ball map to match. */
function fitDetail() {
  const available = ballmap.parentElement.clientWidth - $("ruler-rows").offsetWidth;
  const scale = DETAIL_SCALES.find((s) => detailSize * s <= available) ?? DETAIL_SCALES.at(-1);
  detailCanvas.style.width = `${detailSize * scale}px`;
  detailCanvas.style.height = `${detailSize * scale}px`;
  ballmap.style.setProperty("--plate", `${detailSize * scale}px`);
  ballmap.style.setProperty("--cell", `${state.cell * scale}px`);
  ballmap.style.setProperty("--margin", `${state.margin * scale}px`);
  const pitch = state.cell * scale;
  ballmap.classList.toggle("is-dense", pitch < MIN_LABEL_PX);
  ballmap.classList.toggle("is-sparse", state.selected.side >= 10 && pitch < MIN_TWO_DIGIT_PX);
}

new ResizeObserver(fitDetail).observe(ballmap.parentElement);

/** The seed as a Rust expression for `&[u8]`: a byte string when it is printable ASCII. */
function rustSeed(seed) {
  let escaped = seed.replaceAll("\\", "\\\\").replaceAll('"', '\\"');
  if (/^[\x20-\x7e]*$/.test(seed)) {
    return `b"${escaped}"`;
  }
  escaped = escaped.replaceAll("\n", "\\n").replaceAll("\r", "\\r").replaceAll("\t", "\\t");
  return `"${escaped}".as_bytes()`;
}

function select(generator) {
  state.selected = generator;
  state.cell = generator.cell;
  state.margin = generator.margin;
  state.transparent = generator.transparent;
  for (const [name, tile] of tiles) {
    tile.button.setAttribute("aria-pressed", String(name === generator.name));
  }
  syncOptions();
  renderDetail();
}

// Render options: the number field and its slider edit the same value.
const options = {
  cell: { field: $("cell"), range: $("cell-range"), min: 1, max: 32 },
  margin: { field: $("margin"), range: $("margin-range"), min: 0, max: 32 },
};

function syncOptions() {
  for (const [key, option] of Object.entries(options)) {
    option.field.value = state[key];
    option.range.value = state[key];
    option.field.removeAttribute("aria-invalid");
  }
  $("transparent").checked = state.transparent;
}

for (const [key, option] of Object.entries(options)) {
  const take = (input) => {
    const value = Number(input.value);
    const valid = input.value !== "" && Number.isInteger(value) && value >= option.min && value <= option.max;
    option.field.toggleAttribute("aria-invalid", !valid);
    if (!valid) return;
    state[key] = value;
    if (input === option.range) option.field.value = value;
    else option.range.value = value;
    renderDetail();
  };
  option.field.addEventListener("input", () => take(option.field));
  option.range.addEventListener("input", () => take(option.range));
  // Leaving an out-of-range field puts back the value the picture was drawn with.
  option.field.addEventListener("change", () => {
    option.field.value = state[key];
    option.field.removeAttribute("aria-invalid");
  });
}

$("transparent").addEventListener("change", (event) => {
  state.transparent = event.target.checked;
  renderDetail();
});

seedInput.addEventListener("input", () => {
  state.seed = seedInput.value;
  renderTiles();
  renderDetail();
});

for (const button of document.querySelectorAll("[data-copy]")) {
  let reset;
  button.addEventListener("click", async () => {
    const code = $(button.dataset.copy);
    clearTimeout(reset);
    try {
      await navigator.clipboard.writeText(code.textContent);
      button.textContent = "Copied";
    } catch {
      button.textContent = "Copy failed";
      const range = document.createRange();
      range.selectNodeContents(code);
      const selection = getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
    }
    reset = setTimeout(() => {
      button.textContent = "Copy";
    }, 1500);
  });
}

$("download").addEventListener("click", () => {
  const seed = state.seed.replace(/[^A-Za-z0-9._-]/gu, "_").slice(0, 64);
  const filename = seed ? `gsichtl-${state.selected.name}-${seed}.png` : `gsichtl-${state.selected.name}.png`;
  detailCanvas.toBlob((blob) => {
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = filename;
    link.click();
    URL.revokeObjectURL(url);
  }, "image/png");
});

state.seed = seedInput.value;
$("work").hidden = false;
select(GENERATORS[0]);
renderTiles();
