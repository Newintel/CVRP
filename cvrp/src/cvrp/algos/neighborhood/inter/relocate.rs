use crate::{
    cvrp::{algos::neighborhood::Neighborhood, objects::Truck, CVRP},
    utils::{log, rand, two_different_random},
};

use super::Relocate;

impl Truck {
    fn relocate_inter(&mut self, i: usize, other: &mut Self, j: usize, cvrp: &CVRP) -> bool {
        let client = self.remove_client_from_cvrp(i, cvrp);

        let client = cvrp.get_cvrp_client(client);

        other.insert_client(client, j)
    }
}

impl<'a> Neighborhood for Relocate<'a> {
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

        if truck1.relocate_inter(self.i, truck2, self.j, self.cvrp) {
            cvrp.update_distance();
            return Some(cvrp);
        }

        None
    }

    fn random_solution(&self) -> Option<CVRP> {
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

        if truck1.relocate_inter(i, truck2, j, self.cvrp) {
            return Some(cvrp);
        }

        None
    }
}

impl<'a> Relocate<'a> {
    pub fn new(cvrp: &'a CVRP) -> Self {
        Relocate {
            cvrp,
            truck1: 0,
            truck2: 0,
            i: 1,
            j: 1,
        }
    }
}
