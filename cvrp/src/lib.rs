mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, cvrp!");
}

#[wasm_bindgen]
pub fn test(text: &str) {
    alert(text);
}

#[wasm_bindgen]
pub struct Camion {
    trajet: String,
}

#[wasm_bindgen]
impl Camion {
    pub fn new(t: &str) -> Camion {
        Camion {
            trajet: String::from(t),
        }
    }

    pub fn to_string(self) -> String {
        self.trajet
    }
}
