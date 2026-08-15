use super::rolling_kendall_rank_correlation::RollingKendallRankCorrelation;
#[test]
fn lifecycle() {
    let mut s = RollingKendallRankCorrelation::new(2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 3.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}

#[test]
fn scalar_bulk_chunks_ties_and_continuation_are_bitwise_invariant() {
    let x: Vec<f64> = (0..157)
        .map(|index| ((index * 7) % 19) as f64 + (index as f64 * 0.11).sin())
        .collect();
    let y: Vec<f64> = (0..157)
        .map(|index| ((index * 5) % 13) as f64 - (index as f64 * 0.07).cos())
        .collect();
    let mut scalar = RollingKendallRankCorrelation::new(20).unwrap();
    let expected: Vec<_> = x
        .iter()
        .zip(&y)
        .map(|(&x, &y)| scalar.append(x, y).unwrap_or(f64::NAN))
        .collect();
    let expected_value = scalar.value();
    let expected_continuation = scalar.append(3.0, 9.0);

    for size in [1, 9, x.len()] {
        let mut bulk = RollingKendallRankCorrelation::new(20).unwrap();
        let mut actual = Vec::new();
        for (x, y) in x.chunks(size).zip(y.chunks(size)) {
            bulk.extend_slices_into(x, y, &mut actual).unwrap();
        }
        for (&actual, &expected) in actual.iter().zip(&expected) {
            assert!(
                actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan())
            );
        }
        assert_eq!(bulk.value(), expected_value);
        assert_eq!(bulk.append(3.0, 9.0), expected_continuation);
    }
}

#[test]
fn validation_precedes_mutation() {
    assert!(RollingKendallRankCorrelation::new(1).is_err());
    let mut state = RollingKendallRankCorrelation::new(3).unwrap();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[3.0], &mut Vec::new())
        .is_err());
    assert_eq!(state.value(), None);
    assert_eq!(state.append(1.0, 2.0), None);
}
