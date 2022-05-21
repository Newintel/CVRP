use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn clear(ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
    ctx.clear_rect(
        0.into(),
        0.into(),
        canvas.width().into(),
        canvas.height().into(),
    );
}

#[wasm_bindgen]
pub enum Color {
    BLACK,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    PURPLE,
    CYAN,
}
