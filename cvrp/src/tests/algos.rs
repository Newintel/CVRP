#[cfg(test)]
mod algos {
    use crate::cvrp::objects::truck::Truck;
    use test_case::test_case;

    #[test]
    fn test_echange() {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        for _ in 0..10 {
            truck.exchange();
            assert_eq!(truck.route[0], 0);
            assert_eq!(truck.route[truck.route.len() - 1], 0);
        }
    }

    #[test_case(2, 5; "start lower than end")]
    #[test_case(5, 2; "start higher than end")]
    fn test_inversion(start: usize, end: usize) {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        truck._inversion(start, end);
        assert_eq!(truck.route, vec![0, 1, 3, 4, 5, 6, 0]);
    }

    #[test_case(2, 5; "start lower than end")]
    #[test_case(5, 2; "start higher than end")]
    fn test_insertion_shift(start: usize, end: usize) {
        let mut truck = Truck::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        truck._insertion_shift(start, end);
        assert_eq!(truck.route, vec![0, 1, 3, 6, 5, 4, 0]);
    }
}
