pub mod full_neighborhood;
pub mod inter;
pub mod intra;

use std::usize;

use strum::{EnumString, FromRepr};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::cvrp::CVRP;

pub trait Neighborhood {
    fn has_next(&self) -> bool;
    fn next_indexes(&mut self);
    fn create_new(&self) -> Option<CVRP>;
    fn random_solution(&mut self) -> Option<CVRP>;

    fn next(&mut self) -> Option<CVRP> {
        let mut cvrp = None;

        self.next_indexes();

        if self.has_next() {
            cvrp = self.create_new();
        }

        cvrp
    }
}

pub struct FullNeighborhood<'a> {
    components: Vec<&'a mut dyn Neighborhood>,
    index: usize,
}

#[wasm_bindgen]
#[derive(FromRepr, PartialEq, Eq, Hash, EnumString)]
pub enum NeighborhoodStruct {
    InterExchange,
    IntraExchange,
    InterRelocate,
    IntraRelocate,
}
