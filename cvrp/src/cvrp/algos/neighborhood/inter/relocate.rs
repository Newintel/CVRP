use crate::{
    cvrp::{algos::neighborhood::Neighborhood, objects::Truck, CVRP},
    utils::rand,
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
        len > 1 && self.truck2 < len
    }

    fn next_indexes(&mut self) {
        self.j += 1;

        let truck = self.cvrp.trucks.get(self.truck2).unwrap();
        if self.j == truck.route.len() + 1 {
            self.i += 1;
            self.j = 1;
        }

        let truck = self.cvrp.trucks.get(self.truck1).unwrap();
        if self.i == truck.route.len() {
            self.truck1 += 1;
            self.j = 1;
            self.i = 1;
        }

        if self.truck1 == self.truck2 {
            self.truck1 += 1;
        }

        if self.truck1 == self.cvrp.trucks.len() {
            self.truck2 += 1;
            self.truck1 = 0;
        }
    }

    fn create_new(&self) -> Option<CVRP> {
        let mut cvrp = self.cvrp.clone();

        let (mut t1, mut t2) = (self.truck1, self.truck2);

        if t1 > t2 {
            (t1, t2) = (t2, t1);
        }

        let trucks = cvrp.trucks.as_mut_slice().split_at_mut(t2);
        let mut truck2 = trucks.1.get_mut(0).unwrap();
        let mut truck1 = trucks.0.get_mut(t1).unwrap();

        if self.truck1 > self.truck2 {
            (truck2, truck1) = (truck1, truck2);
        }

        if truck1.relocate_inter(self.i, truck2, self.j, self.cvrp) {
            cvrp.update_distance();
            return Some(cvrp);
        }

        None
    }

    fn random_solution(&mut self) -> Option<CVRP> {
        let mut all = vec![];
        let max = ((self.cvrp.clients.len() - 1) as f64).powi(2) as u128;
        let r = rand(max, None);
        let mut i = 0;
        while self.has_next() {
            let next = self.next();
            if next.is_some() {
                self.nb_sol += 1;
                if i == r {
                    return next;
                }
                all.push(next);
            }
            i += 1;
        }

        let r = rand(all.len(), None);

        return all.remove(r);
    }

    fn get_nb_sol(&self) -> u128 {
        self.nb_sol
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
            nb_sol: 0,
        }
    }
}
