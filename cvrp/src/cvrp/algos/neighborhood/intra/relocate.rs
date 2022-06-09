use crate::{
    cvrp::{algos::neighborhood::Neighborhood, objects::Truck, CVRP},
    utils::{log, rand, two_different_random},
};

use super::Relocate;

impl Truck {
    fn relocate(&mut self, i: usize, j: usize) {
        let client = self.route.remove(j);
        self.route.insert(i, client);
        self.must_update = true;
    }
}

impl<'a> Neighborhood for Relocate<'a> {
    fn has_next(&self) -> bool {
        self.truck < self.cvrp.trucks.len()
    }

    fn next_indexes(&mut self) {
        let len = self.cvrp.trucks.get(self.truck).unwrap().route.len();

        self.j += 1;

        if self.j == len {
            self.i += 1;
            self.j = self.i;
        }

        if self.i == len {
            self.truck += 1;
            self.i = 1;
            self.j = 1;
            if self.has_next() {
                self.next_indexes();
            }
        }
    }

    fn create_new(&self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();
        let truck = cvrp.trucks.get_mut(self.truck).unwrap();
        truck.relocate(self.i, self.j);
        cvrp.update_distance();
        Some(cvrp)
    }

    fn random_solution(&self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();
        let truck = cvrp
            .trucks
            .get_mut(rand(self.cvrp.trucks.len() - 1, None) + 1)
            .unwrap();
        let len = truck.route.len();

        if len == 2 {
            return None;
        }

        let (i, j) = two_different_random(len - 1);
        let (i, j) = (i + 1, j + 1);

        truck.relocate(i, j);

        Some(cvrp)
    }
}

impl<'a> Relocate<'a> {
    pub fn new(cvrp: &'a CVRP) -> Self {
        Relocate {
            cvrp,
            truck: 0,
            i: 1,
            j: 1,
        }
    }
}
