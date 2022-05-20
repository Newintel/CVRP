use std::fmt;

use wasm_bindgen::JsValue;

use super::client::Client;

#[derive(Clone)]
pub struct Truck {
    pub route: Vec<i16>,
    pub poids: i32,
    pub poids_max: i32,
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
    pub fn new(poids_max: i32) -> Self {
        Truck {
            route: vec![],
            poids: 0,
            poids_max,
        }
    }

    pub fn mock(route: Option<Vec<i16>>, poids: Option<i32>, poids_max: Option<i32>) -> Self {
        Truck {
            route: route.unwrap_or(vec![]),
            poids: poids.unwrap_or(0),
            poids_max: poids_max.unwrap_or(0),
        }
    }

    pub fn add_client(&mut self, c: &Client) -> bool {
        if self.poids + c.q as i32 <= self.poids_max {
            self.route.push(c.i);
            self.poids += c.q as i32;
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

    pub fn insert_client_in_route(&mut self, index: usize, client: i16) {
        self.route.insert(index, client)
    }

    pub fn insert_clients_in_route(&mut self, index: usize, clients: Vec<i16>) {
        let route = vec![
            self.route[..index].to_vec(),
            clients,
            self.route[index..].to_vec(),
        ]
        .concat();
        self.route = route;
    }

    pub fn remove_clients_in_route(&mut self, start: usize, end: usize) -> Vec<i16> {
        let route = self.route[start..=end].to_vec();
        self.route = vec![
            self.route[..start].to_vec(),
            self.route[(end + 1)..].to_vec(),
        ]
        .concat();
        route
    }
}
