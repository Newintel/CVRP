use std::fmt;

use crate::cvrp::Weight;

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
        }
    }

    pub fn add_client(&mut self, c: &Client) -> bool {
        if self.weight + c.q as Weight <= self.max_weight {
            self.route.push(c.i);
            self.weight += c.q as Weight;
            return true;
        }
        false
    }

    pub fn get_full_route(&self) -> Vec<Index> {
        let mut route = vec![0];
        for i in &self.route {
            route.push(*i);
        }
        route.push(0);
        route
    }
}
