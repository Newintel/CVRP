use std::f64::consts::PI;

use strum::{EnumCount, IntoEnumIterator};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::cvrp::{display::utils::clear, objects::Index, Distance, CVRP};

use super::utils::Color;

const POINT_SIZE: f64 = 3f64;

#[wasm_bindgen]
impl CVRP {
    fn coordinates(&self, client: Index) -> (Distance, Distance) {
        let client = self
            .get_cvrp_client(client)
            .to_display(self.factor, self.offset);
        (client.x.into(), client.y.into())
    }

    pub fn display(&self, ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
        clear(ctx, canvas);
        for client in &self.clients {
            let client = client.to_display(self.factor, self.offset);
            let color;
            if client.is_source() {
                color = Color::RED;
            } else {
                color = Color::BLACK;
            }
            ctx.set_fill_style(&JsValue::from(color.as_str()));

            ctx.begin_path();
            let ok = ctx.arc(
                client.x.into(),
                client.y.into(),
                POINT_SIZE,
                0f64,
                2f64 * PI,
            );
            ok.expect("Arc draw failed");

            if client.i != 0 {
                ctx.set_font("4px");
                let ok = ctx.fill_text(
                    client.i.to_string().as_str(),
                    (client.x + 4).into(),
                    client.y.into(),
                );
                ok.expect("Text draw failed");
            }

            ctx.fill();
        }
    }

    pub fn display_path(&self, ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
        self.display(ctx, canvas);
        for (index, truck) in self.trucks.iter().enumerate() {
            ctx.begin_path();
            let len: i16 = truck.route.len() as i16;

            for i in 0..len {
                let client = *truck.route.get(i as usize).unwrap();
                let (x, y) = self.coordinates(client);
                ctx.move_to(x.into(), y.into());
                let client2 = *truck.route.get(((i + 1) % len) as usize).unwrap();
                let (x, y) = self.coordinates(client2);
                ctx.line_to(x.into(), y.into());
            }

            let colors: Vec<Color> = Color::iter().collect();
            let color = colors.get(index % Color::COUNT).unwrap_or(&Color::BLACK);

            ctx.set_stroke_style(&JsValue::from(color.as_str()));
            ctx.stroke();
        }
    }
}
