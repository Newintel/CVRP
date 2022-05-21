mod algos;
mod display;
pub mod objects;

use csv::{self, ReaderBuilder};
use js_sys::{self};
use wasm_bindgen::prelude::*;

use self::objects::{client::Client, truck::Truck};

const DEFAULT_N_NEIGHBORS: usize = 20;
const DEFAULT_N_ITERATIONS: u16 = 1000;

#[wasm_bindgen]
#[derive(Clone)]
pub struct CVRP {
    clients: Vec<Client>,
    pub total_weight: u16,
    max_truck_weight: u16,
    trucks: Vec<Truck>,
    pub n_neighbors: usize,
    pub n_iterations: u16,
    pub factor: u8,
}

impl CVRP {
    pub fn get_cvrp_client(&self, index: u8) -> &Client {
        return self
            .clients
            .iter()
            .find(|cli| cli.i == index)
            .expect("Index not found");
    }
    pub fn mock(
        clients: Option<Vec<Client>>,
        trucks: Option<Vec<Truck>>,
        max_truck_weight: Option<u16>,
        total_weight: Option<u16>,
    ) -> Self {
        return Self {
            clients: clients.unwrap_or(vec![]),
            trucks: trucks.unwrap_or(vec![]),
            max_truck_weight: max_truck_weight.unwrap_or(0),
            total_weight: total_weight.unwrap_or(0),
            n_neighbors: DEFAULT_N_NEIGHBORS,
            n_iterations: DEFAULT_N_ITERATIONS,
            factor: 1,
        };
    }

    pub fn get_trucks(&self) -> &Vec<Truck> {
        &self.trucks
    }
}

#[wasm_bindgen]
impl CVRP {
    pub fn new(
        max_truck_weight: u16,
        factor: Option<u8>,
        n_neighbors: Option<usize>,
        n_iterations: Option<u16>,
    ) -> CVRP {
        CVRP {
            clients: Vec::new(),
            total_weight: 0,
            max_truck_weight,
            trucks: vec![],
            n_neighbors: n_neighbors.unwrap_or(DEFAULT_N_NEIGHBORS),
            n_iterations: n_iterations.unwrap_or(DEFAULT_N_ITERATIONS),
            factor: factor.unwrap_or(1),
        }
    }

    pub fn read_data(&mut self, csv: String) {
        self.clients.clear();
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv.as_bytes());

        for entry in reader.deserialize::<Client>() {
            let record = entry.expect("Entry as Client failed");
            self.total_weight += record.q as u16;
            self.clients.push(record);
        }
    }

    pub fn get_clients(&self) -> js_sys::Array {
        let clients = js_sys::Array::new();
        for client in &self.clients {
            clients.push(&client.to_json());
        }
        clients
    }

    pub fn get_routes(&self) -> js_sys::Array {
        let routes = js_sys::Array::new();
        for truck in &self.trucks {
            routes.push(&JsValue::from(truck.get_route()));
        }
        routes
    }

    pub fn get_client(&self, index: u8) -> JsValue {
        return self.get_cvrp_client(index).to_json();
    }
}

impl CVRP {
    pub fn get_max_nb_truck(&self) -> u8 {
        ((self.total_weight / self.max_truck_weight) as u8)
            + ((self.total_weight % self.max_truck_weight != 0) as u8)
    }

    pub fn get_distance(&self) -> f64 {
        let mut distance = 0_f64;
        for truck in &self.trucks {
            let mut i = 0;
            let len = truck.route.len();
            while i < len - 1 {
                distance += self.clients[i].distance(&self.clients[i + 1]);
                i += 1;
            }
        }
        distance
    }
}

impl PartialEq for CVRP {
    fn eq(&self, other: &Self) -> bool {
        let len = self.trucks.len();
        if len != other.trucks.len() {
            return false;
        }

        let mut i = 0;
        while i < len && self.trucks.get(i).unwrap().route != other.trucks.get(i).unwrap().route {
            i += 1;
        }

        return i == len;
    }
}
