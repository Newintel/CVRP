use crate::cvrp::objects::truck::Truck;

impl Truck {
    pub fn _insertion_shift(&mut self, start: usize, end: usize) {
        let val = self.route[end];
        for i in (start..end).rev() {
            self.route[i + 1] = *self.route.get(i).unwrap();
        }
        self.route[start] = val;
    }

    pub fn _inversion(&mut self, start: usize, mut end: usize) {
        end += 1;
        let stop = (start + end) / 2;
        for i in start..stop {
            self.route.swap(i, start + end - 1 - i);
        }
    }
}
