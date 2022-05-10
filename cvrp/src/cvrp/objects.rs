use serde::Deserialize;
use serde_json::json;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
#[wasm_bindgen]
pub struct Client {
    i: i32,
    x: i32,
    y: i32,
    q: i32,
}

#[wasm_bindgen]
pub struct Camion {
    trajet: Vec<Client>,
}

#[wasm_bindgen]
impl Client {
    pub fn new(i: i32, x: i32, y: i32, q: i32) -> Client {
        Client { i, x, y, q }
    }

    pub fn to_point(&self) -> JsValue {
        JsValue::from_serde(&json!({"x" : self.i, "y" : self.y})).expect("JSON failed")
    }

    pub fn to_string(self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.i, self.x, self.y, self.q)
    }
}

#[wasm_bindgen]
impl Camion {
    pub fn new() -> Camion {
        Camion { trajet: Vec::new() }
    }

    pub fn add_client(&mut self, c: Client) {
        self.trajet.push(c);
    }

    // fn to_string(self) -> String {
    //     let mut string : String = String::from("");
    //     for client in self.trajet {
    //         string = format!("{} -> {}", string, client);
    //     }
    //     string
    // }
}

impl fmt::Display for Camion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::from("Camion: ");
        for client in &self.trajet {
            string = format!("{} -> {}", string, client);
        }
        write!(f, "{}", string)
    }
}
