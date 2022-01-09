mod utils;

use rand::random;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::swap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console::info_1;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::Window;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Tick {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
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

impl Tick {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        let rising = self.open <= self.close;

        let color = if rising { "red" } else { "green" };
        context.set_fill_style(&JsValue::from_str(color));
        context.set_stroke_style(&JsValue::from_str(color));

        let line_top = self.high * 100.0;
        let line_bottom = self.low * 100.0;
        let mut box_top = self.open * 100.0;
        let mut box_bottom = self.close * 100.0;

        if rising {
            swap(&mut box_top, &mut box_bottom);
        }

        let middle = 50.0;
        let width = 20.0;

        context.clear_rect(0.0, 0.0, 300.0, 200.0);
        context.begin_path();
        context.move_to(middle, line_top);
        context.line_to(middle, line_bottom);
        context.stroke();

        context.fill_rect(middle - width / 2.0, box_top, width, box_top - box_bottom);
    }
}

fn random_data() -> Tick {
    let mut data = vec![
        random::<f64>(),
        random::<f64>(),
        random::<f64>(),
        random::<f64>(),
    ];

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let falling = random::<bool>();

    let high = data[3];
    let low = data[0];
    let mut open = data[1];
    let mut close = data[2];

    if falling {
        swap(&mut open, &mut close);
    }

    Tick {
        open,
        close,
        high,
        low,
    }
}

fn render(window: &Window, context: &CanvasRenderingContext2d) {
    let tick = random_data();
    tick.draw(&context);
}

fn start(window: &Window) -> Option<()> {
    let doc = window.document()?;
    let elem = doc.get_element_by_id("canvas")?;
    let canvas = elem.dyn_into::<HtmlCanvasElement>().ok()?;
    let context = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;

    render(&window, &context);
    Some(())
}

#[wasm_bindgen]
pub fn run() {
    if let Some(window) = web_sys::window() {
        start(&window);
    }
}
