use std::f64::consts::PI;

use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::cvrp::{display::utils::clear, objects::Index, Distance, CVRP};

use super::utils::Color;

const POINT_SIZE: f64 = 3f64;

#[wasm_bindgen]
impl CVRP {
    fn coordinates(&self, client: Index) -> (Distance, Distance) {
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
            let ok = ctx.arc(
                client.x.into(),
                client.y.into(),
                POINT_SIZE,
                0f64,
                2f64 * PI,
            );
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
            let len: i16 = truck.route.len() as i16;

            for i in 0..len {
                let client = *truck.route.get(i as usize).unwrap_throw();
                let (x, y) = self.coordinates(client);
                ctx.move_to(x.into(), y.into());
                let client2 = *truck.route.get(((i + 1) % len) as usize).unwrap_throw();
                let (x, y) = self.coordinates(client2);
                ctx.line_to(x.into(), y.into());
            }

            ctx.set_stroke_style(&colors.get((index as u32) % colors.length()));
            ctx.stroke();
        }
    }
}
