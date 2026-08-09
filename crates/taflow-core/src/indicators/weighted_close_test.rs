use super::weighted_close::WeightedClose;

#[test]
fn lifecycle_bulk_and_validation_are_consistent() {
    let high = [12.0_f64, 13.0, 14.0, 15.0];
    let low = [8.0_f64, 9.0, 10.0, 11.0];
    let close = [11.0_f64, 12.0, 13.0, 14.0];
    let expected: Vec<_> = (0..high.len())
        .map(|index| {
            let high = high[index];
            let low = low[index];
            let close = close[index];
            (high + low + close + close) * 0.25
        })
        .collect();
    let mut state = WeightedClose::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = (0..high.len())
        .map(|index| state.append(high[index], low[index], close[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(state.value(), expected.last().copied());
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state
        .extend_slices_into(&high, &low, &close, &mut bulk)
        .unwrap();
    assert_eq!(bulk, expected);
    let before = bulk.clone();
    assert!(state
        .extend_slices_into(&high, &low, &close[..3], &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
