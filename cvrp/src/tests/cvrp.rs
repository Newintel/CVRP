#[cfg(test)]
mod cvrp {
    use crate::cvrp::CVRP;
    use test_case::test_case;

    #[test_case(200, 100; "When mod is 0")]
    #[test_case(200, 110; "When mod is not 0")]
    fn test_max_camions(max_weight: i32, max_camion_weight: i32) {
        let cvrp = CVRP::mock(None, None, Some(max_camion_weight), Some(max_weight));
        assert_eq!(2, cvrp.get_max_nb_camion())
    }
}