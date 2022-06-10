use crate::cvrp::{algos::neighborhood::Neighborhood, Truck, CVRP};
use instant::Instant;
use rand::prelude::*;
use std::{cmp::min, str::FromStr};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::neighborhood::{
    inter::{Exchange as InterExchange, Relocate as InterRelocate},
    intra::{Exchange, Relocate},
    FullNeighborhood, NeighborhoodStruct,
};

#[wasm_bindgen]
impl CVRP {
    pub fn random_solution(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        display_info: &js_sys::Function,
    ) {
        let time = Instant::now();
        self.trucks.clear();
        for _ in 0..self.get_max_nb_truck() {
            let mut truck = Truck::new(self.max_truck_weight);
            truck.add_client(self.get_cvrp_client(0));
            self.trucks.push(truck);
        }

        let mut n_clients: Vec<usize> = (1..self.clients.len()).collect();

        let mut rng = rand::thread_rng();
        n_clients.shuffle(&mut rng);

        for index in n_clients {
            let mut i = 0;
            let client = self.clients.get(index).unwrap();

            while self.trucks.get_mut(i).unwrap().add_client(client) == false {
                i += 1;
                if i == self.trucks.len() {
                    let mut truck = Truck::new(self.max_truck_weight);
                    truck.add_client(self.get_cvrp_client(0));
                    self.trucks.push(truck);
                }
            }
        }

        self.update_distance();

        self.display_infos(display_info, time.elapsed(), 0);

        self.display_path(ctx, canvas);
    }

    pub fn tabu_search(
        &mut self,
        max_tabu_size: usize,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        n_iterations: u16,
        display_info: &js_sys::Function,
        neighborhood_struct: &js_sys::Array,
    ) -> Self {
        let mut best = self.clone();
        best.random_solution(&ctx, &canvas, display_info);

        let time = Instant::now();

        let mut tabu: Vec<Self> = vec![best.clone()];

        let mut best_choice = best.clone();

        let neighborhood_structs: Vec<NeighborhoodStruct> = neighborhood_struct
            .iter()
            .filter_map(|j| {
                let neigh = j.as_string().expect("Not a string");
                Some(
                    NeighborhoodStruct::from_str(neigh.as_str())
                        .expect(format!("Not a neighborhood, {neigh}").as_str()),
                )
            })
            .collect();

        let mut nb_sol = 0;

        for _ in 0..n_iterations {
            let mut components: Vec<&mut dyn Neighborhood> = vec![];
            let mut local_best = best_choice.clone();

            let exchange = &mut Exchange::new(&best);
            let inter_exchange = &mut InterExchange::new(&best);
            let relocate = &mut Relocate::new(&best);
            let inter_relocate = &mut InterRelocate::new(&best);

            if neighborhood_structs.contains(&NeighborhoodStruct::IntraExchange) {
                components.push(exchange);
            }
            if neighborhood_structs.contains(&NeighborhoodStruct::InterExchange) {
                components.push(inter_exchange);
            }
            if neighborhood_structs.contains(&NeighborhoodStruct::IntraRelocate) {
                components.push(relocate);
            }
            if neighborhood_structs.contains(&NeighborhoodStruct::InterRelocate) {
                components.push(inter_relocate);
            }

            let mut neighborhood = FullNeighborhood::new(components);

            while neighborhood.has_next() {
                let next = neighborhood.next();
                if next.is_some() {
                    let next = next.unwrap();
                    if tabu.contains(&next) == false {
                        local_best = min(local_best, next);
                    }
                }
            }

            nb_sol += neighborhood.get_nb_sol();

            if local_best >= best_choice {
                tabu.push(best_choice.clone());
            } else {
                best_choice = local_best;
            }

            best = min(best, best_choice.clone());

            if tabu.len() > max_tabu_size {
                tabu.remove(0);
            }
        }

        best.display_infos(display_info, time.elapsed(), nb_sol);

        best.display_path(ctx, canvas);

        best
    }

    pub fn simulated_annealing(
        &mut self,
        initial_temperature: f64,
        t_changes: u32,
        mu: f64,
        iterations_per_t: u16,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        display_info: &js_sys::Function,
        neighborhood_struct: &js_sys::Array,
    ) -> Self {
        let mut best = self.clone();
        best.random_solution(ctx, canvas, display_info);

        let time = Instant::now();

        let mut local_best = best.clone();
        let mut t = initial_temperature;

        let neighborhood_structs: Vec<NeighborhoodStruct> = neighborhood_struct
            .iter()
            .filter_map(|j| {
                let neigh = j.as_string().expect("Not a string");
                Some(
                    NeighborhoodStruct::from_str(neigh.as_str())
                        .expect(format!("Not a neighborhood, {neigh}").as_str()),
                )
            })
            .collect();

        let mut nb_sol = 0;

        for _ in 0..t_changes {
            for _ in 0..iterations_per_t {
                let mut components: Vec<&mut dyn Neighborhood> = vec![];

                let exchange = &mut Exchange::new(&best);
                let inter_exchange = &mut InterExchange::new(&best);
                let relocate = &mut Relocate::new(&best);
                let inter_relocate = &mut InterRelocate::new(&best);

                if neighborhood_structs.contains(&NeighborhoodStruct::IntraExchange) {
                    components.push(exchange);
                }
                if neighborhood_structs.contains(&NeighborhoodStruct::InterExchange) {
                    components.push(inter_exchange);
                }
                if neighborhood_structs.contains(&NeighborhoodStruct::IntraRelocate) {
                    components.push(relocate);
                }
                if neighborhood_structs.contains(&NeighborhoodStruct::InterRelocate) {
                    components.push(inter_relocate);
                }

                let mut neighborhood = FullNeighborhood::new(components);

                let y = neighborhood.random_solution().unwrap();
                nb_sol += neighborhood.get_nb_sol();

                let delta = y.distance - local_best.distance;
                if delta <= 0.into() {
                    local_best = y;
                    best = min(best, local_best.clone());
                } else {
                    let r = rand::random::<f64>();
                    if r < f64::exp(-delta / t) {
                        local_best = y;
                    }
                }
            }
            t = mu * t;
        }

        best.display_infos(display_info, time.elapsed(), nb_sol);

        best.display_path(ctx, canvas);

        best.clone()
    }
}
