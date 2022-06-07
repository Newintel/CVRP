#[cfg(test)]
mod cvrp {
    use crate::cvrp::{
        objects::{Client, Index, Truck},
        Weight, CVRP,
    };
    use test_case::test_case;

    #[test_case(vec![1, 2, 3, 4, 5], 5; "When mod is 0")]
    #[test_case(vec![1, 2, 3, 4, 6], 6; "When mod is not 0")]
    fn test_max_trucks(clients: Vec<Index>, max_truck_weight: Weight) {
        let cvrp = CVRP::mock(
            Some(Client::mock_many(clients)),
            None,
            Some(max_truck_weight),
        );
        assert_eq!(3, cvrp.get_max_nb_truck())
    }

    #[test]
    fn test_cvrp_eq() {
        let c1 = CVRP::mock(
            None,
            Some(vec![
                Truck::mock(Some(vec![1, 2, 3]), None, None),
                Truck::mock(Some(vec![4, 5, 6, 7]), None, None),
            ]),
            None,
        );
        let c2 = CVRP::mock(
            None,
            Some(vec![
                Truck::mock(Some(vec![1, 2, 3]), None, None),
                Truck::mock(Some(vec![4, 5, 6, 7]), None, None),
            ]),
            None,
        );

        assert!(c1 == c2);
    }

    #[test]
    fn test_cvrp_vec_contains() {
        let c1 = CVRP::mock(
            None,
            Some(vec![
                Truck::mock(Some(vec![1, 2, 3]), None, None),
                Truck::mock(Some(vec![4, 5, 6, 7]), None, None),
            ]),
            None,
        );
        let c2 = CVRP::mock(
            None,
            Some(vec![
                Truck::mock(Some(vec![1, 2, 3]), None, None),
                Truck::mock(Some(vec![4, 5, 6, 7]), None, None),
            ]),
            None,
        );

        let v = vec![c1];
        assert!(v.contains(&c2));
    }
}
