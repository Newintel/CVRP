mod methods;
mod utils;

use crate::cvrp::objects::truck::Truck;
use crate::cvrp::CVRP;

// Transformation locale
impl Truck {
    pub fn exchange(&mut self) {
        let (i, j) = self.take_indexes_except_source();
        self.route.swap(i, j);
    }

    pub fn inversion(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_shift(start, end)
    }

    pub fn insertion_shift(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_shift(start, end)
    }
}

impl CVRP {
    pub fn cross_exchange(&mut self) {
        let (i1, i2) = self.two_random_trucks();
        let mut c1 = self.trucks.get(i1).unwrap().to_owned();
        let mut c2 = self.trucks.get(i2).unwrap().to_owned();
        let (j1, clients1) = c1.remove_random_clients();
        let (j2, clients2) = c2.remove_random_clients();
        c1.insert_clients_in_route(j1, clients2);
        c2.insert_clients_in_route(j2, clients1);
        self.trucks[i1] = c1;
        self.trucks[i2] = c2;
    }

    pub fn exchange(&mut self) {
        let (i1, i2) = self.two_random_trucks();
        let mut c1 = self.trucks.get(i1).unwrap().to_owned();
        let mut c2 = self.trucks.get(i2).unwrap().to_owned();
        let (j1, client1) = c1.get_random_client();
        let (j2, client2) = c2.get_random_client();
        c1.insert_client_in_route(j1, client2);
        c2.insert_client_in_route(j2, client1);
        self.trucks[i1] = c1;
        self.trucks[i2] = c2;
    }

    // pub fn generate_neighborhood(&self) -> Vec<Self> {}
}
