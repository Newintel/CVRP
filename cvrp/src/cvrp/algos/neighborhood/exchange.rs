use wasm_bindgen::UnwrapThrowExt;

use crate::cvrp::{objects::Truck, CVRP};

use super::{Exchange, Neighborhood};

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
        if (self.i == 0 && self.j == 0) == false {
            self.j += 1;
        }
        if self.j == self.i {
            self.i += 1;
            self.j = 0;
        }
        let truck = self.cvrp.trucks.get(self.truck).unwrap_throw();
        if self.i == truck.route.len() {
            self.truck += 1;
            self.i = 0;
            self.j = 0;
            if self.has_next() {
                self.next_indexes();
            }
        }
    }

    fn create_new(&self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();
        let truck = cvrp.trucks.get_mut(self.truck).unwrap_throw();
        truck.exchange(self.i, self.j);
        cvrp.update_distance();
        Some(cvrp)
    }

    fn next(&mut self) -> Option<CVRP> {
        let mut cvrp = None;

        self.next_indexes();
        if self.has_next() {
            cvrp = self.create_new();
        }

        cvrp
    }
}

impl<'a> Exchange<'a> {
    pub fn new(cvrp: &'a CVRP) -> Self {
        Exchange {
            cvrp,
            truck: 0,
            i: 0,
            j: 0,
        }
    }
}
