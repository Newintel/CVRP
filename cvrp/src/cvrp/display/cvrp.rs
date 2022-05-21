use std::f64::consts::PI;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::cvrp::{display::utils::clear, CVRP};

use super::utils::Color;

#[wasm_bindgen]
impl CVRP {
    fn coordinates(&self, client: u8) -> (f64, f64) {
        let client = self.get_cvrp_client(client).with_factor(self.factor);
        (client.x.into(), client.y.into())
    }

    pub fn display(
        &self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &js_sys::Array,
    ) {
        clear(ctx, canvas);
        for client in &self.clients {
            let client = client.with_factor(self.factor);
            ctx.begin_path();
            let ok = ctx.arc(client.x.into(), client.y.into(), 3f64, 0f64, 2f64 * PI);
            ok.expect("Arc draw failed");
            let color;
            if client.is_source() {
                color = Color::RED;
            } else {
                color = Color::BLACK;
            }
            ctx.set_fill_style(&colors.get(color as u32));
            ctx.fill();
        }
    }

    pub fn display_path(
        &self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &js_sys::Array,
    ) {
        self.display(ctx, canvas, colors);
        for (index, truck) in self.trucks.iter().enumerate() {
            ctx.begin_path();
            let len = truck.route.len();

            for i in 0..(len - 1) {
                let (x, y) = self.coordinates(*truck.route.get(i).unwrap());
                ctx.move_to(x.into(), y.into());
                let (x, y) = self.coordinates(*truck.route.get(i + 1).unwrap());
                ctx.line_to(x.into(), y.into());
            }

            ctx.set_stroke_style(&colors.get((index as u32) % colors.length()));
            ctx.stroke();
        }
    }
}
