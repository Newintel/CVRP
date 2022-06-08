pub mod exchange;
pub mod full_neighborhood;
pub mod inter_exchange;

use std::usize;

use crate::cvrp::CVRP;

pub trait Neighborhood {
    fn has_next(&self) -> bool;
    fn next_indexes(&mut self);
    fn create_new(&self) -> Option<CVRP>;

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

pub struct Exchange<'a> {
    cvrp: &'a CVRP,
    truck: usize,
    i: usize,
    j: usize,
}

pub struct InterExchange<'a> {
    cvrp: &'a CVRP,
    truck1: usize,
    truck2: usize,
    i: usize,
    j: usize,
}
