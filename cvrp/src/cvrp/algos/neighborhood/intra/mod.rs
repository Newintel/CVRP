use crate::cvrp::CVRP;

pub mod exchange;
pub mod relocate;

pub struct Exchange<'a> {
    cvrp: &'a CVRP,
    truck: usize,
    i: usize,
    j: usize,
}
pub struct Relocate<'a> {
    cvrp: &'a CVRP,
    truck: usize,
    i: usize,
    j: usize,
}
