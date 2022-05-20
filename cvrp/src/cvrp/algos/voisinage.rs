use crate::cvrp::objects::camion::Camion;
use crate::cvrp::CVRP;
use crate::utils::rand;

fn two_different_random(max: usize) -> (usize, usize) {
    let i = rand(max, None);
    let j = rand(max, Some(i));
    (i, j)
}

// Transformation locale
impl Camion {
    fn take_indexes_except_source(&self) -> (usize, usize) {
        let (i, j) = two_different_random(self.trajet.len() - 2);
        (i + 1, j + 1)
    }
    pub fn get_random_client(&self) -> (usize, i16) {
        let index = rand(self.trajet.len() - 2, None) + 1;
        (index, *self.trajet.get(index).unwrap())
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
            self.trajet.insert(i + 1, *self.trajet.get(i).unwrap());
        }
        self.trajet[start] = val;
    }

    pub fn insertion_decalage(&mut self) {
        let (start, end) = self.take_indexes_except_source();
        self._insertion_decalage(start, end)
    }

    pub fn set_in_trajet(&mut self, index: usize, client: i16) {
        self.trajet.insert(index, client)
    }
}

impl CVRP {
    pub fn two_random_camions(&self) -> (usize, usize) {
        two_different_random(self.camions.len())
    }
    pub fn echange(&mut self) {
        let (i1, i2) = self.two_random_camions();
        let mut c1 = self.camions.get(i1).unwrap().to_owned();
        let mut c2 = self.camions.get(i2).unwrap().to_owned();
        let (j1, client1) = c1.get_random_client();
        let (j2, client2) = c2.get_random_client();
        c1.set_in_trajet(j1, client2);
        c2.set_in_trajet(j2, client1);
        self.camions.insert(i1, c1);
        self.camions.insert(i2, c2);
    }
}
