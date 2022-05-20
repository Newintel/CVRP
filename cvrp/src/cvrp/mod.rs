mod algos;
pub mod objects;

use csv::{self, ReaderBuilder};
use js_sys::{self};
use wasm_bindgen::prelude::*;

use self::objects::{camion::Camion, client::Client};

#[wasm_bindgen]
#[derive(Clone)]
pub struct CVRP {
    clients: Vec<Client>,
    pub total_weight: i32,
    max_camion_weight: i32,
    camions: Vec<Camion>,
}

impl CVRP {
    pub fn try_get_client(&self, index: i16) -> &Client {
        return self
            .clients
            .iter()
            .find(|cli| cli.i == index)
            .expect("Index not found");
    }
    pub fn mock(
        clients: Option<Vec<Client>>,
        camions: Option<Vec<Camion>>,
        max_camion_weight: Option<i32>,
        total_weight: Option<i32>,
    ) -> Self {
        return Self {
            clients: clients.unwrap_or(vec![]),
            camions: camions.unwrap_or(vec![]),
            max_camion_weight: max_camion_weight.unwrap_or(0),
            total_weight: total_weight.unwrap_or(0),
        };
    }

    pub fn get_camions(&self) -> &Vec<Camion> {
        &self.camions
    }
}

#[wasm_bindgen]
impl CVRP {
    pub fn new(max_camion_weight: i32) -> CVRP {
        CVRP {
            clients: Vec::new(),
            total_weight: 0,
            max_camion_weight,
            camions: vec![],
        }
    }

    pub fn read_data(&mut self, csv: String) {
        self.clients.clear();
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv.as_bytes());

        for entry in reader.deserialize::<Client>() {
            let record = entry.expect("Entry as Client failed");
            self.total_weight += record.q as i32;
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

    pub fn get_trajets(&self) -> js_sys::Array {
        let trajets = js_sys::Array::new();
        for camion in &self.camions {
            trajets.push(&JsValue::from(camion.get_trajet()));
        }
        trajets
    }

    pub fn get_max_nb_camion(&self) -> i8 {
        ((self.total_weight / self.max_camion_weight) as i8)
            + ((self.total_weight % self.max_camion_weight != 0) as i8)
    }

    pub fn get_client(&self, index: i16) -> JsValue {
        return self.try_get_client(index).to_json();
    }

    pub fn get_distance_parcourue(&self) -> f64 {
        let mut distance = 0_f64;
        for camion in &self.camions {
            let mut i = 0;
            let len = camion.trajet.len();
            while i < len - 1 {
                distance += self.clients[i].distance(&self.clients[i + 1]);
                i += 1;
            }
        }
        distance
    }
}
