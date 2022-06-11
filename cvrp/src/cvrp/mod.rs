mod algos;
mod display;
pub mod objects;

use std::{
    cmp::Ordering,
    fmt::{self, Debug},
};

use csv::{self, ReaderBuilder};
use rand::prelude::SliceRandom;
use wasm_bindgen::prelude::*;

use crate::utils::log;

use self::objects::{Client, Index, Truck};

pub type Display = u8;
pub type Weight = u16;
pub type Distance = f64;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CVRP {
    clients: Vec<Client>,
    max_truck_weight: Weight,
    trucks: Vec<Truck>,
    // Display
    pub factor: Display,
    pub offset: Display,
    // Infos
    pub distance: Distance,
}

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
    ) -> Self {
        return Self {
            clients: clients.unwrap_or(vec![]),
            trucks: trucks.unwrap_or(vec![]),
            max_truck_weight: max_truck_weight.unwrap_or(0),
            factor: 1,
            distance: 0.into(),
            offset: 0.into(),
        };
    }

    pub fn get_trucks(&self) -> &Vec<Truck> {
        &self.trucks
    }
}

#[wasm_bindgen]
impl CVRP {
    #[wasm_bindgen(constructor)]
    pub fn new(max_truck_weight: Weight, factor: Option<Display>, offset: Option<Display>) -> CVRP {
        CVRP {
            clients: Vec::new(),
            max_truck_weight,
            trucks: vec![],
            factor: factor.unwrap_or(1),
            offset: offset.unwrap_or(0),
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
            if record.q > self.max_truck_weight {
                panic!(
                    "Le poids du client {} est plus grand que le poids max par camion défini ({})",
                    record.q, self.max_truck_weight
                );
            }
            self.clients.push(record);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.clients.len() == 0
    }
}

impl CVRP {
    fn get_total_weight(&self) -> Weight {
        self.clients.iter().map(|c| c.q).sum()
    }

    pub fn get_min_nb_truck(&self) -> u8 {
        let total_weight = self.get_total_weight();
        ((total_weight / self.max_truck_weight) as u8)
            + ((total_weight % self.max_truck_weight != 0) as u8)
    }

    fn update_distance(&mut self) {
        self.clean();
        self.distance = 0.into();
        let a = self.clone();
        for truck in &mut self.trucks {
            if truck.must_update {
                truck.update_distance(&a);
            }
            self.distance += truck.distance;
        }
    }

    fn fill_random(&mut self) {
        self.trucks.clear();

        let mut n_clients: Vec<usize> = (1..self.clients.len()).collect();

        let mut rng = rand::thread_rng();
        n_clients.shuffle(&mut rng);

        for index in n_clients {
            let mut i: i32 = -1;
            let client = self.clients.get(index).unwrap();

            while i < 0 || self.trucks.get_mut(i as usize).unwrap().add_client(client) == false {
                i += 1;
                let i = i as usize;
                if i == self.trucks.len() {
                    let mut truck = Truck::new(self.max_truck_weight);
                    truck.add_client(self.get_cvrp_client(0));
                    self.trucks.push(truck);
                }
            }
        }

        self.update_distance();
    }

    fn clean(&mut self) {
        let len = self.trucks.len();
        let mut rem: Vec<usize> = vec![];

        for i in 0..len {
            if self.trucks.get(i).unwrap().route.len() < 2 {
                rem.push(i);
            }
        }

        rem.reverse();

        for i in rem {
            self.trucks.remove(i);
        }
    }
}

impl PartialEq for CVRP {
    fn eq(&self, other: &Self) -> bool {
        let len = self.trucks.len();
        if len != other.trucks.len() {
            return false;
        }

        let mut i = 0;
        while i < len && self.trucks.get(i).unwrap() == other.trucks.get(i).unwrap() {
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
            if self.trucks.len() > other.trucks.len() {
                return Ordering::Greater;
            }
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
        write!(f, "CVRP: {} \n -- {:?}", self.distance, self.trucks)
    }
}

impl CVRP {
    pub fn log(&self, pre: Option<&str>) {
        log(format!("{} - {self}", pre.unwrap_or("")))
    }
}
