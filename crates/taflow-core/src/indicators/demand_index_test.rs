use super::demand_index::DemandIndex;

#[test]
fn matches_wickra_reference_values_and_reset_replay() {
    let high = [11.0, 12.0, 11.0, 13.0, 12.0];
    let low = [9.0, 10.0, 9.0, 11.0, 10.0];
    let close = [10.0, 11.0, 10.0, 12.0, 11.0];
    let volume = [100.0, 110.0, 120.0, 130.0, 140.0];
    let expected = [
        None,
        None,
        None,
        Some(10.502479338842975),
        Some(-1.5543158861340682),
    ];
    let mut state = DemandIndex::new(3).unwrap();

    let actual: Vec<_> = high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&high, low), close), volume)| state.append(high, low, close, volume))
        .collect();
    for (actual, expected) in actual.into_iter().zip(expected) {
        match (actual, expected) {
            (Some(actual), Some(expected)) => assert!(
                (actual - expected).abs() < 1e-12,
                "expected {expected}, got {actual}"
            ),
            (actual, expected) => assert_eq!(actual, expected),
        }
    }

    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(high[0], low[0], close[0], volume[0]), None);
}

#[test]
fn validates_period_and_handles_zero_previous_close() {
    assert!(DemandIndex::new(0).is_err());

    let mut state = DemandIndex::new(1).unwrap();
    assert_eq!(state.append(0.0, 0.0, 0.0, 100.0), None);
    assert_eq!(state.append(1.0, 0.0, 1.0, 100.0), Some(0.0));
}
