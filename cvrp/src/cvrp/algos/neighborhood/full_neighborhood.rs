use crate::{cvrp::CVRP, utils::log};

use super::{FullNeighborhood, Neighborhood};

impl<'a> Neighborhood for FullNeighborhood<'a> {
    fn has_next(&self) -> bool {
        self.index < self.components.len()
    }

    fn next_indexes(&mut self) {
        let comp = self.components.get(self.index);
        if comp.is_some() && comp.unwrap().has_next() == false {
            self.index += 1;
        }
    }

    fn create_new(&self) -> Option<CVRP> {
        None
    }

    fn next(&mut self) -> Option<CVRP> {
        self.next_indexes();

        if self.has_next() {
            let neighborhood = self.components.get_mut(self.index);

            if neighborhood.is_some() {
                let neighborhood = neighborhood.unwrap();
                return neighborhood.next();
            }
        }

        return None;
    }
}

impl<'a> FullNeighborhood<'a> {
    pub fn new(components: Vec<&'a mut dyn Neighborhood>) -> Self {
        FullNeighborhood {
            components,
            index: 0,
        }
    }
}
