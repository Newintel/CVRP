#[cfg(test)]
mod algos {
    use std::iter::FromIterator;

    use crate::cvrp::{
        objects::{client::Client, truck::Truck},
        CVRP,
    };

    #[test]
    fn test_exchange() {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        for _ in 0..10 {
            truck.exchange();
            assert_eq!(truck.route[0], 0);
            assert_eq!(truck.route[truck.route.len() - 1], 0);
        }
    }

    #[test]
    fn test_inversion() {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        truck._inversion(2, 5);
        assert_eq!(truck.route, vec![0, 1, 3, 4, 5, 6, 0]);
    }

    #[test]
    fn test_insertion_shift() {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        truck._insertion_shift(2, 5);
        assert_eq!(truck.route, vec![0, 1, 3, 6, 5, 4, 0]);
    }

    #[test]
    fn test_cross_exchange() {
        for _ in 0..10 {
            let truck = Truck::mock(Some(vec![0, 1, 2, 3, 4, 5, 0]), None, Some(16));
            let truck2 = Truck::mock(Some(vec![0, 6, 7, 8, 9, 0]), None, Some(16));

            let clients = Client::mock_many(vec![0, 2, 4, 6, 1, 3, 5, 5, 4, 3]);

            let cvrp = CVRP::mock(Some(clients), Some(vec![truck, truck2]), None, None);
            let mut cvrp2 = cvrp.clone();
            cvrp2.cross_exchange();

            assert_ne!(cvrp, cvrp2);

            let trucks = cvrp2
                .get_trucks()
                .into_iter()
                .map(|t| t.route.to_vec())
                .flatten()
                .filter(|x| *x != 0);

            let mut trucks = Vec::from_iter(trucks);
            trucks.sort();

            assert_eq!(trucks, Vec::from_iter((1..10).into_iter()))
        }
    }
}
