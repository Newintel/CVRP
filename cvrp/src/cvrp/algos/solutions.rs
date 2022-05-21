use crate::{
    cvrp::{Truck, CVRP},
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

        self.display_path(ctx, canvas, colors)
    }

    pub fn tabu_search(
        &mut self,
        max_tabu_size: usize,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
    ) -> Self {
        let mut i = 0;
        let mut tabu: Vec<Self> = vec![];
        let mut best: CVRP = self.clone();
        let mut best_distance = f64::MAX;

        self.random_solution(&ctx, &canvas, &colors);

        while i < self.n_iterations {
            log(format!("iteration {i}/{}", self.n_iterations).as_str());
            let neighbors = self.generate_neighborhood();
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

            let best_candidate = neighbors.get(best_candidate).unwrap().to_owned();

            if best_distance > best_candidate_distance {
                best_distance = best_candidate_distance;
                best = best_candidate.clone();
                if i % 1000 == 0 {
                    best.display_path(ctx, canvas, colors);
                }
            }

            tabu.push(best_candidate);
            if tabu.len() > max_tabu_size {
                tabu.remove(0);
            }

            i += 1;
        }

        best
    }
}
