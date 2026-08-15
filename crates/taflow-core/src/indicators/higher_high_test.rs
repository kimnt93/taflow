use super::higher_high::HigherHigh;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = HigherHigh::new();
    assert_eq!(state.append(10.0, 8.0), None);
    assert_eq!(state.append(11.0, 9.0), Some(1.0));
    assert_eq!(state.value(), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_chunking_and_continuation_match_scalar_replay() {
    let high = [10.0, 11.0, 9.0, 9.0, 12.0, f64::NAN, 13.0];
    let low = [8.0, 9.0, 7.0, 8.0, 10.0, 6.0, 11.0];

    let mut scalar = HigherHigh::new();
    let expected: Vec<_> = high
        .iter()
        .zip(low)
        .map(|(&high, low)| scalar.append(high, low).unwrap_or(f64::NAN))
        .collect();

    let mut bulk = HigherHigh::new();
    let mut actual = Vec::new();
    bulk.extend_slices_into(&high[..3], &low[..3], &mut actual)
        .unwrap();
    bulk.extend_slices_into(&high[3..], &low[3..], &mut actual)
        .unwrap();
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(bulk.value(), scalar.value());
    assert_eq!(bulk.append(14.0, 12.0), scalar.append(14.0, 12.0));

    let before = actual.clone();
    let value = bulk.value();
    assert!(bulk
        .extend_slices_into(&high, &low[..low.len() - 1], &mut actual)
        .is_err());
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        before
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(bulk.value(), value);
}
