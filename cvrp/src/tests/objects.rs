#[cfg(test)]
mod objects {
    mod truck {
        use crate::cvrp::objects::truck::Truck;

        #[test]
        fn test_remove_route() {
            let mut truck = Truck::mock(Some(vec![0, 1, 2, 3, 4, 5, 0]), None, None);
            let route = truck.remove_clients_in_route(2, 5);

            assert_eq!(route, vec![2, 3, 4, 5]);
            assert_eq!(truck.route, vec![0, 1, 0]);
        }

        #[test]
        fn test_add_route() {
            let mut truck = Truck::mock(Some(vec![0, 1, 2, 3, 4, 5, 0]), None, None);
            truck.insert_clients_in_route(2, vec![7, 8, 9]);

            assert_eq!(truck.route, vec![0, 1, 7, 8, 9, 2, 3, 4, 5, 0]);
        }
    }
}
