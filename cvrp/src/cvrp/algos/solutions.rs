use crate::{
    cvrp::{algos::neighborhood::Neighborhood, Truck, CVRP, DEFAULT_N_ITERATIONS},
    utils::log,
};
use js_sys::Array;
use rand::prelude::*;
use std::cmp::max;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::neighborhood::Exchange;

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
            let truck = Truck::new(self.max_truck_weight);
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

        self.update_distance();

        self.display_path(ctx, canvas, colors);
    }

    pub fn tabu_search(
        &mut self,
        max_tabu_size: usize,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
        n_iterations: Option<u16>,
    ) -> Self {
        self.random_solution(&ctx, &canvas, &colors, Some(false));
        let mut i = 0;
        let mut tabu: Vec<Self> = vec![self.clone()];

        let n_iterations = n_iterations.unwrap_or(DEFAULT_N_ITERATIONS);

        let mut best = self.clone();
        let mut best_choice = self.clone();

        best.log(Some("First"));

        while i < n_iterations {
            let mut neighborhood = Exchange::new(tabu.last().unwrap_throw());
            while neighborhood.has_next() {
                let next = neighborhood.next();
                if next.is_some() {
                    let next = next.unwrap_throw();
                    if next.distance > best_choice.distance {
                        best_choice.log(Some("new best"));
                    }
                    if tabu.contains(&next) == false {
                        best_choice = max(best_choice, next);
                    }
                }
            }

            best = max(best, best_choice.clone());

            tabu.push(best_choice.clone());

            if tabu.len() > max_tabu_size {
                tabu.remove(0);
            }

            i += 1;
        }

        best.display_path(ctx, canvas, colors);

        best
    }
}
