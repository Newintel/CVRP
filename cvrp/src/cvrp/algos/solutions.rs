use crate::cvrp::{algos::neighborhood::Neighborhood, Truck, CVRP, DEFAULT_N_ITERATIONS};
use js_sys::Array;
use rand::prelude::*;
use std::cmp::min;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::neighborhood::{Exchange, FullNeighborhood, InterExchange};

#[wasm_bindgen]
impl CVRP {
    pub fn random_solution(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
        display_info: &js_sys::Function,
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
            let client = self.clients.get(index).unwrap();

            while self.trucks[i].add_client(client) == false {
                if i == self.trucks.len() - 1 {
                    let mut truck = Truck::new(self.max_truck_weight);
                    truck.add_client(self.get_cvrp_client(0));
                    self.trucks.push(truck);
                }
                i += 1;
            }
        }

        self.update_distance();

        self.display_path(ctx, canvas, colors);

        display_info
            .call2(
                &JsValue::UNDEFINED,
                &JsValue::from("distance"),
                &JsValue::from(self.distance),
            )
            .err();
    }

    pub fn tabu_search(
        &mut self,
        max_tabu_size: usize,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        colors: &Array,
        n_iterations: Option<u16>,
        display_info: &js_sys::Function,
    ) -> Self {
        // self.clients = self.clients[0..20].to_vec();
        self.random_solution(&ctx, &canvas, &colors, display_info);
        let mut i = 0;
        let mut tabu: Vec<Self> = vec![self.clone()];

        let n_iterations = n_iterations.unwrap_or(DEFAULT_N_ITERATIONS);

        let mut best = self.clone();
        let mut best_choice = self.clone();

        while i < n_iterations {
            let b = tabu.last().unwrap();
            let mut exchange = Exchange::new(b);
            let mut exchange_inter = InterExchange::new(b);

            // let mut neighborhood = FullNeighborhood::new(vec![&mut exchange, &mut exchange_inter]);
            // let mut neighborhood = FullNeighborhood::new(vec![&mut exchange_inter]);
            let mut neighborhood = FullNeighborhood::new(vec![&mut exchange]);

            while neighborhood.has_next() {
                let next = neighborhood.next();
                if next.is_some() {
                    let next = next.unwrap();
                    if tabu.contains(&next) == false {
                        best_choice = min(best_choice, next);
                    }
                }
            }

            best = min(best, best_choice.clone());
            best.display_path(ctx, canvas, colors);

            display_info
                .call2(
                    &JsValue::UNDEFINED,
                    &JsValue::from("distance"),
                    &JsValue::from(self.distance),
                )
                .err();

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
