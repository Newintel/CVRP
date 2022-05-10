mod objects;

use csv::{self, ReaderBuilder};
use js_sys::{self};
use objects::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CVRP {
    clients: Vec<Client>,
}

#[wasm_bindgen]
impl CVRP {
    pub fn new() -> CVRP {
        CVRP {
            clients: Vec::new(),
        }
    }

    pub fn read_data(&mut self, csv: String) {
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv.as_bytes());

        for entry in reader.deserialize::<Client>() {
            let record = entry.expect("nope");
            self.clients.push(record)
        }
    }

    pub fn clients_to_points(&self) -> js_sys::Array {
        let clients = js_sys::Array::new();
        for client in &self.clients {
            clients.push(&client.to_point());
        }
        clients
    }
}
