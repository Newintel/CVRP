mod algos;
mod display;
pub mod objects;

use std::{
    cmp::Ordering,
    fmt::{self, Debug},
};

use csv::{self, ReaderBuilder};
use js_sys::{self};
use wasm_bindgen::prelude::*;

use crate::utils::log;

use self::objects::{Client, Index, Truck};

pub type Weight = u16;
pub type Distance = f64;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CVRP {
    clients: Vec<Client>,
    pub total_weight: Weight,
    max_truck_weight: Weight,
    trucks: Vec<Truck>,
    pub factor: u8,
    pub distance: Distance,
}

pub const DEFAULT_N_ITERATIONS: u16 = 1000;

impl CVRP {
    pub fn get_cvrp_client(&self, index: Index) -> &Client {
        return self
            .clients
            .iter()
            .find(|cli| cli.i == index)
            .expect("Index not found");
    }
    pub fn mock(
        clients: Option<Vec<Client>>,
        trucks: Option<Vec<Truck>>,
        max_truck_weight: Option<Weight>,
        total_weight: Option<Weight>,
    ) -> Self {
        return Self {
            clients: clients.unwrap_or(vec![]),
            trucks: trucks.unwrap_or(vec![]),
            max_truck_weight: max_truck_weight.unwrap_or(0),
            total_weight: total_weight.unwrap_or(0),
            factor: 1,
            distance: 0.into(),
        };
    }

    pub fn get_trucks(&self) -> &Vec<Truck> {
        &self.trucks
    }
}

#[wasm_bindgen]
impl CVRP {
    #[wasm_bindgen(constructor)]
    pub fn new(max_truck_weight: Weight, factor: Option<u8>) -> CVRP {
        CVRP {
            clients: Vec::new(),
            total_weight: 0,
            max_truck_weight,
            trucks: vec![],
            factor: factor.unwrap_or(1),
            distance: 0.into(),
        }
    }

    pub fn read_data(&mut self, csv: String) {
        self.clients.clear();
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv.as_bytes());

        for entry in reader.deserialize::<Client>() {
            let record = entry.expect("Entry as Client failed");
            self.total_weight += record.q as Weight;
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

    pub fn get_client(&self, index: Index) -> JsValue {
        return self.get_cvrp_client(index).to_json();
    }
}

impl CVRP {
    pub fn get_max_nb_truck(&self) -> u8 {
        ((self.total_weight / self.max_truck_weight) as u8)
            + ((self.total_weight % self.max_truck_weight != 0) as u8)
    }

    pub fn update_distance(&mut self) -> Distance {
        let mut distance: Distance = 0.into();
        for truck in &self.trucks {
            let len = truck.route.len();
            for i in 0..(len - 1) {
                distance += self.clients[i].distance(&self.clients[i + 1]);
            }
        }
        self.distance = distance;
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

impl Eq for CVRP {}

impl Ord for CVRP {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance > other.distance {
            return Ordering::Greater;
        }

        if self.distance == other.distance {
            return Ordering::Equal;
        }

        Ordering::Less
    }
}

impl PartialOrd for CVRP {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for CVRP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CVRP: {}", self.distance)
    }
}

impl CVRP {
    pub fn log(&self, pre: Option<&str>) {
        log(format!("{} - {self}", pre.unwrap_or("")).as_str())
    }
}
