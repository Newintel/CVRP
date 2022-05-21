#[cfg(test)]
mod objects {
    mod truck {
        use crate::cvrp::{
            objects::{client::Client, truck::Truck},
            CVRP,
        };

        fn generate_cvrp() -> CVRP {
            CVRP::mock(
                Some(Client::mock_many(vec![0, 2, 1, 6, 2, 10, 3])),
                None,
                Some(21),
                None,
            )
        }

        #[test]
        fn test_remove_route() {
            let cvrp = generate_cvrp();
            let mut truck = Truck::mock(Some(vec![0, 1, 2, 3, 4, 5, 0]), None, None);
            let route = truck.remove_clients_in_route(2, 5, &cvrp);

            assert_eq!(route, vec![2, 3, 4, 5]);
            assert_eq!(truck.route, vec![0, 1, 0]);
        }

        #[test]
        fn test_add_route() {
            let cvrp = generate_cvrp();
            let mut truck = Truck::mock(Some(vec![0, 1, 2, 3, 0]), None, None);
            assert!(truck.insert_clients_in_route(2, vec![4, 5], &cvrp));
            assert!(truck.insert_clients_in_route(2, vec![6], &cvrp) == false);

            assert_eq!(truck.route, vec![0, 1, 4, 5, 2, 3, 0]);
        }
    }
}
