mod methods;
mod utils;

use std::iter::FromIterator;

use crate::cvrp::objects::truck::Truck;
use crate::cvrp::CVRP;
use crate::utils::log;

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
    pub fn cross_exchange(&mut self) -> bool {
        let (i1, i2) = self.get_two_random_trucks();
        let mut c1 = self.trucks.get(i1).unwrap().to_owned();
        let mut c2 = self.trucks.get(i2).unwrap().to_owned();
        let (j1, clients1) = c1.remove_random_clients(self);
        let (j2, clients2) = c2.remove_random_clients(self);
        let mut ins = c1.insert_clients_in_route(j1, clients2, self);
        if ins == false {
            return false;
        }
        ins = c2.insert_clients_in_route(j2, clients1, self);
        if ins == false {
            return false;
        }
        self.trucks[i1] = c1;
        self.trucks[i2] = c2;
        return true;
    }

    pub fn exchange_inter(&mut self) -> bool {
        let (i1, i2) = self.get_two_random_trucks();
        let mut c1 = self.trucks.get(i1).unwrap().to_owned();
        let mut c2 = self.trucks.get(i2).unwrap().to_owned();

        let (j1, client1) = c1.get_random_client(None);

        let client1_weight = self.get_cvrp_client(client1).q;
        let good_clients = Vec::from_iter(
            self.clients
                .iter()
                .filter(|c| c.q <= client1_weight && c.i != 0)
                .map(|c| c.i),
        );

        if good_clients.is_empty() {
            return false;
        }

        let (j2, client2) = c2.get_random_client(Some(good_clients));

        log(format!("exchange {client1} with {client2}").as_str());

        let test = c1.insert_client_in_route(j1, client2, self);
        if test == false {
            return false;
        }
        let test = c2.insert_client_in_route(j2, client1, self);
        if test == false {
            return false;
        }
        self.trucks[i1] = c1;
        self.trucks[i2] = c2;
        true
    }

    pub fn exchange(&mut self) {
        let (i, mut truck) = self.get_random_truck();
        truck.exchange();
        self.trucks[i] = truck;
    }

    pub fn inversion(&mut self) {
        let (i, mut truck) = self.get_random_truck();
        truck.inversion();
        self.trucks[i] = truck;
    }

    pub fn insertion_shift(&mut self) {
        let (i, mut truck) = self.get_random_truck();
        truck.insertion_shift();
        self.trucks[i] = truck;
    }

    pub fn generate_neighborhood(&self, size: usize) -> Vec<Self> {
        let mut neighbors: Vec<Self> = vec![];

        while neighbors.len() < size / 3 {
            neighbors.push(self.create_random_neighbor_intra());
        }

        while neighbors.len() < size {
            let cvrp = self.create_random_neighbor_inter(false);
            if cvrp.is_some() {
                neighbors.push(cvrp.unwrap());
            } else {
                // log(format!("get neighbor failed: {}", neighbors.len()).as_str())
            }
        }

        neighbors
    }
}
