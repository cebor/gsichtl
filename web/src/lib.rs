//! Runs gsichtl in the browser for the playground at https://cebor.github.io/gsichtl/.

use wasm_bindgen::prelude::*;

/// An avatar rendered for the page.
#[wasm_bindgen]
pub struct Picture {
    kind: gsichtl::Kind,
    size: u32,
    rgba: Vec<u8>,
}

#[wasm_bindgen]
impl Picture {
    /// What was drawn, lowercase (`monster`, `nerd`, `robot`, `crest`, `badge`, `realm`, `retro`):
    /// tells which one `face`, `face12` and `icon` picked.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.kind).to_lowercase()
    }

    /// Width and height in pixels.
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> u32 {
        self.size
    }

    /// RGBA8, row-major, straight alpha. Consumes the picture, so read `kind` and `size` first.
    pub fn into_rgba(self) -> Vec<u8> {
        self.rgba
    }
}

/// Renders `generator`'s avatar for the UTF-8 bytes of `seed`, like
/// `gsichtl::<generator>(seed.as_bytes()).to_rgba(cell_px, margin_px)`.
#[wasm_bindgen]
pub fn render(
    generator: &str,
    seed: &str,
    cell_px: u8,
    margin_px: u8,
    transparent: bool,
) -> Result<Picture, JsError> {
    let draw: fn(&[u8]) -> gsichtl::Avatar = match generator {
        "face" => gsichtl::face,
        "monster" => gsichtl::monster,
        "nerd" => gsichtl::nerd,
        "robot" => gsichtl::robot,
        "face12" => gsichtl::face12,
        "monster12" => gsichtl::monster12,
        "nerd12" => gsichtl::nerd12,
        "robot12" => gsichtl::robot12,
        "crest" => gsichtl::crest,
        "badge" => gsichtl::badge,
        "realm" => gsichtl::realm,
        "retro" => gsichtl::retro,
        "icon" => gsichtl::icon,
        _ => return Err(JsError::new(&format!("unknown generator: {generator}"))),
    };
    let avatar = draw(seed.as_bytes());
    let image = if transparent {
        avatar.to_rgba_transparent(cell_px, margin_px)
    } else {
        avatar.to_rgba(cell_px, margin_px)
    };
    Ok(Picture {
        kind: avatar.kind(),
        size: image.width,
        rgba: image.pixels,
    })
}

/// The gsichtl version that draws the pictures, e.g. `0.5.0`.
#[wasm_bindgen]
pub fn version() -> String {
    env!("GSICHTL_VERSION").to_owned()
}
