use std::{collections::HashMap, iter::FromIterator};

use strum::{EnumCount, EnumIter, IntoEnumIterator};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::cvrp::{Distance, CVRP};

#[wasm_bindgen]
#[derive(EnumCount, EnumIter)]
pub enum Labels {
    Distance,
    NbCamion,
    CamMoyDist,
    MeanTruckWeight,
}

impl Labels {
    pub fn as_str(&self) -> &str {
        match self {
            Labels::Distance => "Distance",
            Labels::NbCamion => "Nombre de camions",
            Labels::CamMoyDist => "Distance moyenne des camions",
            Labels::MeanTruckWeight => "Poids moyen des camions",
        }
    }
}

impl CVRP {
    fn get_infos(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();

        let nb_camion = self.trucks.len();

        map.insert(
            Labels::Distance.as_str().to_string(),
            self.distance.to_string(),
        );
        map.insert(Labels::NbCamion.as_str().to_string(), nb_camion.to_string());
        map.insert(
            Labels::CamMoyDist.as_str().to_string(),
            (self.trucks.iter().map(|t| t.distance).sum::<Distance>() / nb_camion as Distance)
                .to_string(),
        );
        map.insert(
            Labels::MeanTruckWeight.as_str().to_string(),
            (self
                .trucks
                .iter()
                .map(|t| t.weight as Distance)
                .sum::<Distance>()
                / nb_camion as Distance)
                .to_string(),
        );

        map
    }

    pub fn display_infos(&mut self, display_info: &js_sys::Function) {
        self.update_distance();
        let infos = self.get_infos();

        for (label, value) in infos.iter() {
            display_info
                .call2(
                    &JsValue::UNDEFINED,
                    &JsValue::from(label),
                    &JsValue::from(value),
                )
                .err();
        }
    }
}

#[wasm_bindgen]
pub fn all_labels() -> js_sys::Array {
    js_sys::Array::from_iter(Labels::iter().map(|label| JsValue::from(label.as_str())))
}
