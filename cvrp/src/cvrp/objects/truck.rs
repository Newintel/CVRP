use std::fmt;

use wasm_bindgen::UnwrapThrowExt;

use crate::cvrp::{Weight, CVRP};

use super::{Client, Index, Truck};

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
    pub fn new(poids_max: Weight) -> Self {
        Truck {
            route: vec![],
            weight: 0,
            max_weight: poids_max,
            distance: 0.into(),
            must_update: false,
        }
    }

    pub fn mock(
        route: Option<Vec<Index>>,
        poids: Option<Weight>,
        poids_max: Option<Weight>,
    ) -> Self {
        Truck {
            route: route.unwrap_or(vec![]),
            weight: poids.unwrap_or(0),
            max_weight: poids_max.unwrap_or(0),
            distance: 0.into(),
            must_update: false,
        }
    }

    pub fn add_client(&mut self, c: &Client) -> bool {
        if self.weight + c.q as Weight <= self.max_weight {
            self.route.push(c.i);
            self.weight += c.q as Weight;
            self.must_update = true;
            return true;
        }
        false
    }

    pub fn update_distance(&mut self, cvrp: &CVRP) {
        self.distance = 0.into();
        let len: Index = self.route.len();
        for i in 0..len {
            let c1 = cvrp.get_cvrp_client(*self.route.get(i).unwrap_throw());
            let c2 = cvrp.get_cvrp_client(*self.route.get((i + 1) % len).unwrap_throw());

            self.distance += c1.distance(c2);
        }
        self.must_update = false;
    }
}

impl PartialEq for Truck {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}
