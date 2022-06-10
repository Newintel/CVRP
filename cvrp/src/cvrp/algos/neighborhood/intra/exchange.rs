use crate::{
    cvrp::{algos::neighborhood::Neighborhood, objects::Truck, CVRP},
    utils::{rand, two_different_random},
};

use super::Exchange;

impl Truck {
    fn exchange(&mut self, i: usize, j: usize) {
        self.route.swap(i, j);
        self.must_update = true;
    }
}

impl<'a> Neighborhood for Exchange<'a> {
    fn has_next(&self) -> bool {
        self.truck < self.cvrp.trucks.len()
    }

    fn next_indexes(&mut self) {
        let len = self.cvrp.trucks.get(self.truck).unwrap().route.len();

        if (self.i == 1 && self.j == 1) == false {
            self.j += 1;
        }

        if self.j == self.i {
            self.i += 1;
            self.j = 1;
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
        truck.exchange(self.i, self.j);
        cvrp.update_distance();
        Some(cvrp)
    }

    fn random_solution(&mut self) -> Option<CVRP> {
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

        truck.exchange(i, j);

        Some(cvrp)
    }
}

impl<'a> Exchange<'a> {
    pub fn new(cvrp: &'a CVRP) -> Self {
        Exchange {
            cvrp,
            truck: 0,
            i: 1,
            j: 1,
        }
    }
}
