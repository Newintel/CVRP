use std::{fmt, iter::FromIterator};

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen]
pub struct Client {
    pub i: u8,
    pub x: u16,
    pub y: u16,
    pub q: u8,
}

impl Client {
    pub fn new(i: u8, x: u16, y: u16, q: u8) -> Self {
        Client { i, x, y, q }
    }

    pub fn mock(i: u8, q: u8) -> Self {
        Client { i, x: 0, y: 0, q }
    }

    pub fn mock_many(list: Vec<u8>) -> Vec<Self> {
        Vec::from_iter(
            list.iter()
                .enumerate()
                .map(|(i, p)| Self::mock(i as u8, *p)),
        )
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&json!({"i" : self.i, "x" : self.x, "y" : self.y, "q" : self.q}))
            .expect("Client to JSON failed")
    }

    pub fn distance(&self, client: &Self) -> f64 {
        f64::sqrt(((self.x - client.x).pow(2) + (self.y - client.y).pow(2)) as f64)
    }

    pub fn with_factor(&self, factor: u8) -> Client {
        Client::new(
            self.i,
            self.x * (factor as u16),
            self.y * (factor as u16),
            self.q,
        )
    }

    pub fn is_source(&self) -> bool {
        self.i == 0
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.i, self.x, self.y, self.q)
    }
}
