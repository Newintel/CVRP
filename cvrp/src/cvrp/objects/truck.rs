use std::fmt;

use wasm_bindgen::JsValue;

use crate::cvrp::CVRP;

use super::client::Client;

#[derive(Clone, Debug)]
pub struct Truck {
    pub route: Vec<u8>,
    pub weight: u16,
    pub max_weight: u16,
}

impl fmt::Display for Truck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::from("Truck: ");
        for client in &self.route {
            string = format!("{} -> {}", string, client);
        }
        write!(f, "{}", string)
    }
}

impl Truck {
    pub fn new(poids_max: u16) -> Self {
        Truck {
            route: vec![],
            weight: 0,
            max_weight: poids_max,
        }
    }

    pub fn mock(route: Option<Vec<u8>>, poids: Option<u16>, poids_max: Option<u16>) -> Self {
        Truck {
            route: route.unwrap_or(vec![]),
            weight: poids.unwrap_or(0),
            max_weight: poids_max.unwrap_or(0),
        }
    }

    pub fn add_client(&mut self, c: &Client) -> bool {
        if self.weight + c.q as u16 <= self.max_weight {
            self.route.push(c.i);
            self.weight += c.q as u16;
            return true;
        }
        false
    }

    pub fn get_route(&self) -> js_sys::Array {
        let route = js_sys::Array::new();
        for i in &self.route {
            route.push(&JsValue::from(*i));
        }
        if self.route.len() > 0 {
            route.push(&JsValue::from(self.route[0]));
        }
        route
    }

    pub fn insert_client_in_route(&mut self, index: usize, client: u8, cvrp: &CVRP) -> bool {
        let old = cvrp.get_cvrp_client(*self.route.get(index).unwrap());
        let new = cvrp.get_cvrp_client(client);
        let new_weight = self.weight - ((old.q - new.q) as u16);
        if new_weight <= self.max_weight {
            self.route[index] = client;
            self.weight = new_weight;
            return true;
        }
        false
    }

    pub fn insert_clients_in_route(&mut self, index: usize, clients: Vec<u8>, cvrp: &CVRP) -> bool {
        let weight: u16 = clients
            .iter()
            .map(|p| cvrp.get_cvrp_client(*p).q as u16)
            .sum();

        if self.weight + weight > cvrp.max_truck_weight {
            return false;
        }

        let route = vec![
            self.route[..index].to_vec(),
            clients,
            self.route[index..].to_vec(),
        ]
        .concat();
        self.route = route;
        true
    }

    pub fn remove_clients_in_route(&mut self, start: usize, end: usize, cvrp: &CVRP) -> Vec<u8> {
        let route = self.route[start..=end].to_vec();
        let sum: u16 = route
            .iter()
            .map(|p| cvrp.get_cvrp_client(*p).q as u16)
            .sum();
        self.weight -= sum;
        self.route = vec![
            self.route[..start].to_vec(),
            self.route[(end + 1)..].to_vec(),
        ]
        .concat();
        route
    }
}
