use std::{convert::TryInto, fmt, iter::FromIterator};

use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::cvrp::{Display, Distance, Weight};

use super::{Client, Coord, Index};

impl Client {
    pub fn new(i: Index, x: Coord, y: Coord, q: Weight) -> Self {
        Client { i, x, y, q }
    }

    pub fn mock(i: Index, q: Weight) -> Self {
        Client { i, x: 0, y: 0, q }
    }

    pub fn mock_many(list: Vec<Index>) -> Vec<Self> {
        Vec::from_iter(
            list.iter()
                .enumerate()
                .map(|(i, p)| Self::mock(i, (*p).try_into().unwrap())),
        )
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&json!({"i" : self.i, "x" : self.x, "y" : self.y, "q" : self.q}))
            .expect("Client to JSON failed")
    }

    pub fn distance(&self, client: &Self) -> Distance {
        Distance::sqrt(((self.x - client.x).pow(2) + (self.y - client.y).pow(2)).into())
    }

    pub fn to_display(&self, factor: Display, offset: Display) -> Client {
        Client::new(
            self.i,
            self.x * (factor as Coord) + (offset / 2) as Coord,
            self.y * (factor as Coord) + (offset / 2) as Coord,
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
