use crate::{
    cvrp::{objects::truck::Truck, CVRP},
    utils::{rand, two_different_random},
};

impl Truck {
    pub fn take_indexes_except_source(&self) -> (usize, usize) {
        let (mut start, mut end) = two_different_random(self.route.len() - 2);
        if start > end {
            (start, end) = (end, start);
        }
        (start + 1, end + 1)
    }

    pub fn get_random_client(&self) -> (usize, i16) {
        let index = rand(self.route.len() - 2, None) + 1;
        (index, *self.route.get(index).unwrap())
    }

    pub fn remove_random_clients(&mut self) -> (usize, Vec<i16>) {
        let (start, end) = self.take_indexes_except_source();
        (start, self.remove_clients_in_route(start, end))
    }
}

impl CVRP {
    pub fn two_random_trucks(&self) -> (usize, usize) {
        two_different_random(self.trucks.len())
    }
}
