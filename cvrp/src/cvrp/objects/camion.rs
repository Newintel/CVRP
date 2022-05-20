use std::fmt;

use wasm_bindgen::JsValue;

use super::client::Client;

#[derive(Clone)]
pub struct Camion {
    pub trajet: Vec<i16>,
    pub poids: i32,
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
    pub fn new(poids_max: i32) -> Self {
        Camion {
            trajet: vec![],
            poids: 0,
            poids_max,
        }
    }

    pub fn mock(trajet: Option<Vec<i16>>, poids: Option<i32>, poids_max: Option<i32>) -> Self {
        Camion {
            trajet: trajet.unwrap_or(vec![]),
            poids: poids.unwrap_or(0),
            poids_max: poids_max.unwrap_or(0),
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
