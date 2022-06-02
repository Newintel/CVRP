use std::iter::FromIterator;

use wasm_bindgen::throw_str;

use crate::{
    cvrp::{objects::truck::Truck, CVRP},
    utils::{log, rand, two_different_random},
};

impl Truck {
    pub fn take_indexes_except_source(&self) -> (usize, usize) {
        let (mut start, mut end) = two_different_random(self.route.len() - 2);
        if start > end {
            (start, end) = (end, start);
        }
        (start + 1, end + 1)
    }

    pub fn get_random_client(&self, amongst: Option<Vec<u8>>) -> (usize, u8) {
        let route;
        if amongst.is_some() {
            route = Vec::from_iter(
                self.route
                    .iter()
                    .filter(|c| amongst.as_ref().unwrap().contains(*c)),
            );
        } else {
            route = Vec::from_iter(self.route.iter().filter(|c| **c != 0));
        }
        let index = rand(route.len(), None);
        let client = **route.get(index).unwrap();
        (
            self.route.iter().position(|c| *c == client).unwrap(),
            client,
        )
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
    pub fn create_random_neighbor_intra(&self) -> CVRP {
        let r = rand::<u8>(3, None);
        let mut cvrp = self.clone();

        match r {
            0 => cvrp.exchange(),
            1 => cvrp.inversion(),
            2 => cvrp.insertion_shift(),
            _ => throw_str(&format!("Random not supposed to be {r}").to_string()),
        }

        log(format!(
            "r : {r}, dist : {}, orig : {}",
            cvrp.get_distance(),
            self.get_distance()
        )
        .as_str());
        cvrp
    }

    pub fn create_random_neighbor_inter(&self, with_cross_exchange: bool) -> Option<CVRP> {
        let r = if with_cross_exchange {
            rand::<u8>(2, None)
        } else {
            0
        };

        let mut cvrp = self.clone();

        match r {
            0 => {
                if cvrp.exchange_inter() == false {
                    return None;
                }
            }
            1 => {
                if cvrp.cross_exchange() == false {
                    return None;
                }
            }
            _ => throw_str(&format!("Random not supposed to be {r}").to_string()),
        }

        Some(cvrp)
    }
}
