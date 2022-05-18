use std::fmt;

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
#[wasm_bindgen]
pub struct Client {
    pub i: i8,
    pub x: i8,
    pub y: i8,
    pub q: i8,
}

impl Client {
    pub fn new(i: i8, x: i8, y: i8, q: i8) -> Client {
        Client { i, x, y, q }
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&json!({"i" : self.i, "x" : self.x, "y" : self.y, "q" : self.q}))
            .expect("Client to JSON failed")
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.i, self.x, self.y, self.q)
    }
}
