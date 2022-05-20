#[cfg(test)]
mod algos {
    use crate::cvrp::objects::camion::Camion;
    use test_case::test_case;

    #[test]
    fn test_echange() {
        let mut camion = Camion::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        for _ in 0..10 {
            camion.echange();
            assert_eq!(camion.trajet[0], 0);
            assert_eq!(camion.trajet[camion.trajet.len() - 1], 0);
        }
    }

    #[test_case(2, 5; "start lower than end")]
    #[test_case(5, 2; "start higher than end")]
    fn test_inversion(start: usize, end: usize) {
        let mut camion = Camion::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        camion._inversion(start, end);
        assert_eq!(camion.trajet, vec![0, 1, 3, 4, 5, 6, 0]);
    }

    #[test_case(2, 5; "start lower than end")]
    #[test_case(5, 2; "start higher than end")]
    fn test_insertion_decalage(start: usize, end: usize) {
        let mut camion = Camion::mock(Some(vec![0, 1, 6, 5, 4, 3, 0]), None, None);
        camion._insertion_decalage(start, end);
        assert_eq!(camion.trajet, vec![0, 1, 3, 6, 5, 4, 0]);
    }
}
