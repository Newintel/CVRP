use wasm_bindgen::throw_str;

use crate::{
    cvrp::{objects::truck::Truck, CVRP},
    utils::{rand, two_different_random},
};

impl Truck {
    pub fn take_indexes_except_source(&self) -> (usize, usize) {
        let (mut start, mut end) = two_different_random(self.route.len() - 2);
        if start > end {
            (start, end) = (end, start);
        }
        (start + 1, end + 1)
    }

    pub fn get_random_client(&self) -> (usize, u8) {
        let index = rand(self.route.len() - 2, None) + 1;
        (index, *self.route.get(index).unwrap())
    }

    pub fn remove_random_clients(&mut self, cvrp: &CVRP) -> (usize, Vec<u8>) {
        let (start, end) = self.take_indexes_except_source();
        (start, self.remove_clients_in_route(start, end, cvrp))
    }
}

impl CVRP {
    pub fn get_two_random_trucks(&self) -> (usize, usize) {
        two_different_random(self.trucks.len())
    }

    pub fn get_random_truck(&self) -> (usize, Truck) {
        let r = rand(self.trucks.len(), None);
        (r, self.trucks.get(r).unwrap().to_owned())
    }

    pub fn create_random_neighbor(&self) -> Option<CVRP> {
        let r = rand::<u8>(5, None);
        let mut cvrp = self.clone();

        match r {
            0 => cvrp.exchange(),
            1 => cvrp.exchange_inter(),
            2 => {
                if cvrp.cross_exchange() == false {
                    return None;
                }
            }
            3 => cvrp.inversion(),
            4 => cvrp.insertion_shift(),
            _ => throw_str(&format!("Random not supposed to be {r}").to_string()),
        }

        Some(cvrp)
    }
}
