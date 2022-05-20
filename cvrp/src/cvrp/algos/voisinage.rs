use crate::cvrp::objects::camion::Camion;
use crate::utils::rand;

// Transformation locale
impl Camion {
    fn take_indexes_except_source(&self) -> (usize, usize) {
        let len = self.trajet.len() - 2;
        let i = rand(len, None) + 1;
        let j = rand(len, Some(i)) + 1;
        (i, j)
    }

    pub fn echange(&mut self) {
        let (i, j) = self.take_indexes_except_source();
        self.trajet.swap(i, j);
    }

    pub fn _inversion(&mut self, mut start: usize, mut end: usize) {
        if start > end {
            (start, end) = (end, start);
        }
        end += 1;
        let stop = (start + end) / 2;
        for i in start..stop {
            self.trajet.swap(i, start + end - 1 - i);
        }
    }

    pub fn inversion(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_decalage(start, end)
    }
    pub fn _insertion_decalage(&mut self, mut start: usize, mut end: usize) {
        if start > end {
            (start, end) = (end, start);
        }
        let val = self.trajet[end];
        for i in (start..end).rev() {
            self.trajet[i + 1] = self.trajet[i];
        }
        self.trajet[start] = val;
    }

    pub fn insertion_decalage(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_decalage(start, end)
    }
}
