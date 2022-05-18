use std::fmt;

use wasm_bindgen::JsValue;

use super::client::Client;

pub struct Camion {
    trajet: Vec<i8>,
    pub poids: i32,
    pub distance_parcourue: i32,
    pub poids_max: i32,
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

impl Camion {
    pub fn new(poids_max: i32) -> Camion {
        Camion {
            trajet: vec![],
            poids: 0,
            distance_parcourue: 0,
            poids_max,
        }
    }

    pub fn add_client(&mut self, c: &Client) -> bool {
        if self.poids + c.q as i32 <= self.poids_max {
            self.trajet.push(c.i);
            self.poids += c.q as i32;
            return true;
        }
        false
    }

    pub fn get_trajet(&self) -> js_sys::Array {
        let trajet = js_sys::Array::new();
        for i in &self.trajet {
            trajet.push(&JsValue::from(*i));
        }
        if self.trajet.len() > 0 {
            trajet.push(&JsValue::from(self.trajet[0]));
        }
        trajet
    }
}
