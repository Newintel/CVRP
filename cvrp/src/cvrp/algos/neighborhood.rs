use crate::cvrp::objects::truck::Truck;
use crate::cvrp::CVRP;
use crate::utils::rand;

fn two_different_random(max: usize) -> (usize, usize) {
    let i = rand(max, None);
    let j = rand(max, Some(i));
    (i, j)
}

// Transformation locale
impl Truck {
    fn take_indexes_except_source(&self) -> (usize, usize) {
        let (i, j) = two_different_random(self.route.len() - 2);
        (i + 1, j + 1)
    }
    pub fn get_random_client(&self) -> (usize, i16) {
        let index = rand(self.route.len() - 2, None) + 1;
        (index, *self.route.get(index).unwrap())
    }

    pub fn exchange(&mut self) {
        let (i, j) = self.take_indexes_except_source();
        self.route.swap(i, j);
    }

    pub fn _inversion(&mut self, mut start: usize, mut end: usize) {
        if start > end {
            (start, end) = (end, start);
        }
        end += 1;
        let stop = (start + end) / 2;
        for i in start..stop {
            self.route.swap(i, start + end - 1 - i);
        }
    }

    pub fn inversion(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_shift(start, end)
    }
    pub fn _insertion_shift(&mut self, mut start: usize, mut end: usize) {
        if start > end {
            (start, end) = (end, start);
        }
        let val = self.route[end];
        for i in (start..end).rev() {
            self.route.insert(i + 1, *self.route.get(i).unwrap());
        }
        self.route[start] = val;
    }

    pub fn insertion_shift(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_shift(start, end)
    }

    pub fn set_in_route(&mut self, index: usize, client: i16) {
        self.route.insert(index, client)
    }
}

impl CVRP {
    pub fn two_random_trucks(&self) -> (usize, usize) {
        two_different_random(self.trucks.len())
    }
    pub fn echange(&mut self) {
        let (i1, i2) = self.two_random_trucks();
        let mut c1 = self.trucks.get(i1).unwrap().to_owned();
        let mut c2 = self.trucks.get(i2).unwrap().to_owned();
        let (j1, client1) = c1.get_random_client();
        let (j2, client2) = c2.get_random_client();
        c1.set_in_route(j1, client2);
        c2.set_in_route(j2, client1);
        self.trucks.insert(i1, c1);
        self.trucks.insert(i2, c2);
    }
}
