use crate::cvrp::CVRP;

pub mod exchange;
pub mod relocate;

pub struct Exchange<'a> {
    cvrp: &'a CVRP,
    truck1: usize,
    i: usize,
    truck2: usize,
    j: usize,
}

pub struct Relocate<'a> {
    cvrp: &'a CVRP,
    truck1: usize,
    i: usize,
    truck2: usize,
    j: usize,
    nb_sol: u128,
}
