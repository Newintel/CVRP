use crate::cvrp::{Camion, CVRP};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl CVRP {
    pub fn random_solution(&mut self) {
        self.camions.clear();
        for _ in 0..self.get_max_nb_camion() {
            let mut camion = Camion::new(self.max_camion_weight);
            camion.add_client(self.try_get_client(0));
            self.camions.push(camion);
        }

        let mut n_clients: Vec<usize> = (1..self.clients.len()).collect();

        let mut rng = rand::thread_rng();
        n_clients.shuffle(&mut rng);

        for index in n_clients {
            let mut i = 0;
            let client = self.clients.get(index).unwrap_throw();

            while self.camions[i].add_client(client) == false {
                if i == self.camions.len() - 1 {
                    self.camions.push(Camion::new(self.max_camion_weight));
                }
                i += 1;
            }
        }
    }
}
