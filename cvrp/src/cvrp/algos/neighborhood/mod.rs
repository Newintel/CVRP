pub mod exchange;

use std::usize;

use crate::cvrp::CVRP;

pub trait Neighborhood {
    fn has_next(&self) -> bool;
    fn next_indexes(&mut self);
    fn create_new(&self) -> Option<CVRP>;
    fn next(&mut self) -> Option<CVRP>;
}

pub struct Exchange<'a> {
    cvrp: &'a CVRP,
    truck: usize,
    i: usize,
    j: usize,
}
