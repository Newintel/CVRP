use crate::cvrp::{Truck, CVRP};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl CVRP {
    pub fn random_solution(&mut self) {
        self.trucks.clear();
        for _ in 0..self.get_max_nb_truck() {
            let mut truck = Truck::new(self.max_truck_weight);
            truck.add_client(self.try_get_client(0));
            self.trucks.push(truck);
        }

        let mut n_clients: Vec<usize> = (1..self.clients.len()).collect();

        let mut rng = rand::thread_rng();
        n_clients.shuffle(&mut rng);

        for index in n_clients {
            let mut i = 0;
            let client = self.clients.get(index).unwrap_throw();

            while self.trucks[i].add_client(client) == false {
                if i == self.trucks.len() - 1 {
                    self.trucks.push(Truck::new(self.max_truck_weight));
                }
                i += 1;
            }
        }
    }
}
