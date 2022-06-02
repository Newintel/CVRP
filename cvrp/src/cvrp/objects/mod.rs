use serde::Deserialize;
use wasm_bindgen::prelude::*;

use super::Weight;

pub mod client;
pub mod truck;

pub type Coord = u16;
pub type Index = u8;

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen]
pub struct Client {
    pub i: Index,
    pub x: Coord,
    pub y: Coord,
    pub q: Weight,
}

#[derive(Clone, Debug)]
pub struct Truck {
    pub route: Vec<Index>,
    pub weight: Weight,
    pub max_weight: Weight,
}
