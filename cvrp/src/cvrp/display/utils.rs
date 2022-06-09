use strum::{EnumCount, EnumIter};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn clear(ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
    ctx.clear_rect(
        0.into(),
        0.into(),
        canvas.width().into(),
        canvas.height().into(),
    );
}

#[derive(EnumCount, EnumIter)]
pub enum Color {
    BLACK,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    PURPLE,
    CYAN,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::BLACK => "#000",
            Color::RED => "#F00",
            Color::GREEN => "#0F0",
            Color::BLUE => "#00F",
            Color::YELLOW => "#FF0",
            Color::PURPLE => "#F0F",
            Color::CYAN => "#0FF",
        }
    }
}
