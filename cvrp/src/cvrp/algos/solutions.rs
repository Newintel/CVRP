use std::time::Duration;

use crate::{
    cvrp::{Truck, CVRP, DEFAULT_N_ITERATIONS, DEFAULT_N_NEIGHBORS},
    utils::log,
};
use js_sys::Array;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
impl CVRP {
    pub fn random_solution(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
        display: Option<bool>,
    ) {
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
            let client = self.clients.get(index).unwrap_throw();

            while self.trucks[i].add_client(client) == false {
                if i == self.trucks.len() - 1 {
                    self.trucks.push(Truck::new(self.max_truck_weight));
                }
                i += 1;
            }
        }

        for i in 0..self.trucks.len() {
            self.trucks[i].route.push(0);
        }

        self.display_path(ctx, canvas, colors);
    }

    pub fn tabu_search(
        &mut self,
        max_tabu_size: usize,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
        n_iterations: Option<u16>,
        n_neighbors: Option<usize>,
    ) -> Self {
        self.random_solution(&ctx, &canvas, &colors, Some(false));
        let mut i = 0;
        let mut tabu: Vec<Self> = vec![self.clone()];

        let n_iterations = n_iterations.unwrap_or(DEFAULT_N_ITERATIONS);
        let n_neighbors = n_neighbors.unwrap_or(DEFAULT_N_NEIGHBORS);

        let mut best: CVRP = self.clone();
        let mut best_distance = self.get_distance();

        while i < n_iterations {
            // log("{i} / {n_iterations}");
            let neighbors = tabu.last().unwrap().generate_neighborhood(n_neighbors);
            let (mut best_candidate, mut best_candidate_distance) = (0, f64::MAX);

            for (index, candidate) in neighbors.iter().enumerate() {
                if tabu.contains(candidate) == false {
                    let distance = candidate.get_distance();

                    if distance < best_candidate_distance {
                        best_candidate = index;
                        best_candidate_distance = distance;
                    }
                }
            }

            // log(
            //     format!("best distance : {best_candidate_distance}, best : {best_distance}")
            //         .as_str(),
            // );

            let best_candidate = neighbors.get(best_candidate).unwrap().to_owned();
            log(format!("best candidate distance: {best_candidate_distance}").as_str());
            if best_distance > best_candidate_distance {
                best_distance = best_candidate_distance;
                best = best_candidate.clone();
            }

            if i % 1000 == 0 {
                // best.display_path(ctx, canvas, colors);
                log(format!(
                    "iteration : {i}, distance : {best_distance}, {} camions",
                    best.trucks.len()
                )
                .as_str())
            }

            tabu.push(best_candidate);

            if tabu.len() > max_tabu_size {
                tabu.remove(0);
            }

            i += 1;
        }

        best.display_path(ctx, canvas, colors);

        best
    }
}
