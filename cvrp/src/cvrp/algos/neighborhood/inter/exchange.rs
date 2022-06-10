use crate::{
    cvrp::{algos::neighborhood::Neighborhood, objects::Truck, CVRP},
    utils::{rand, two_different_random},
};

use super::Exchange;

impl Truck {
    fn exchange_inter(&mut self, i: usize, other: &mut Self, j: usize, cvrp: &CVRP) -> bool {
        let c1 = self.remove_client_from_cvrp(i, cvrp);
        let c2 = other.remove_client_from_cvrp(j, cvrp);

        let c1 = cvrp.get_cvrp_client(c1);
        let c2 = cvrp.get_cvrp_client(c2);

        other.insert_client(c1, j) && self.insert_client(c2, i)
    }
}

impl<'a> Neighborhood for Exchange<'a> {
    fn has_next(&self) -> bool {
        let len = self.cvrp.trucks.len();
        len > 1
            && self.truck1 < len
            && self.i < self.cvrp.trucks.get(self.truck1).unwrap().route.len()
    }

    fn next_indexes(&mut self) {
        self.j += 1;

        let truck = self.cvrp.trucks.get(self.truck2).unwrap();
        if self.j == truck.route.len() {
            self.i += 1;
            self.j = 1;
        }

        let truck = self.cvrp.trucks.get(self.truck1).unwrap();
        if self.i == truck.route.len() {
            self.truck2 += 1;
            self.j = 1;
            self.i = 1;
        }

        if self.truck1 == self.truck2 {
            self.truck2 = 0;
            self.truck1 += 1;
            self.i = 1;
            self.j = 1;
        }
    }

    fn create_new(&self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();

        let trucks = cvrp.trucks.as_mut_slice().split_at_mut(self.truck1);
        let truck1 = trucks.1.get_mut(0).unwrap();
        let truck2 = trucks.0.get_mut(self.truck2).unwrap();

        if truck1.exchange_inter(self.i, truck2, self.j, self.cvrp) {
            cvrp.update_distance();
            return Some(cvrp);
        }

        None
    }

    fn random_solution(&mut self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();
        let (mut i, mut j) = two_different_random(cvrp.trucks.len() - 1);

        if i > j {
            (i, j) = (j, i);
        }

        let trucks = cvrp.trucks.as_mut_slice().split_at_mut(j);
        let truck1 = trucks.1.get_mut(0).unwrap();
        let truck2 = trucks.0.get_mut(i).unwrap();

        let i = rand(truck1.route.len() - 1, None) + 1;
        let j = rand(truck2.route.len() - 1, None) + 1;

        if truck1.exchange_inter(i, truck2, j, self.cvrp) {
            return Some(cvrp);
        }

        None
    }
}

impl<'a> Exchange<'a> {
    pub fn new(cvrp: &'a CVRP) -> Self {
        Exchange {
            cvrp,
            truck1: 0,
            truck2: 0,
            i: 1,
            j: 1,
        }
    }
}
