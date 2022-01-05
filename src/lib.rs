mod utils;

use rand::random;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Tick {
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}

impl Display for Tick {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "open: {} close: {} high: {} close: {}",
            self.open, self.close, self.high, self.low
        )
    }
}

fn random_data() -> Tick {
    Tick {
        open: random(),
        close: random(),
        high: random(),
        low: random(),
    }
}

fn render() -> Option<()> {
    let window = web_sys::window()?;
    let doc = window.document()?;
    let elem = doc.get_element_by_id("content")?;
    let tick = random_data();
    elem.set_inner_html(&tick.to_string());
    Some(())
}

#[wasm_bindgen]
pub fn run() {
    render();
}
