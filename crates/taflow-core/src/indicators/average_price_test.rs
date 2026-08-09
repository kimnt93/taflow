use super::average_price::AveragePrice;

#[test]
fn lifecycle_bulk_and_validation_are_consistent() {
    let open = [10.0_f64, 11.0, 12.0, 13.0];
    let high = [12.0_f64, 13.0, 14.0, 15.0];
    let low = [8.0_f64, 9.0, 10.0, 11.0];
    let close = [11.0_f64, 12.0, 13.0, 14.0];
    let expected: Vec<_> = (0..open.len())
        .map(|index| {
            let open = open[index];
            let high = high[index];
            let low = low[index];
            let close = close[index];
            (open + high + low + close) * 0.25
        })
        .collect();
    let mut state = AveragePrice::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = (0..open.len())
        .map(|index| state.append(open[index], high[index], low[index], close[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(state.value(), expected.last().copied());
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state
        .extend_slices_into(&open, &high, &low, &close, &mut bulk)
        .unwrap();
    assert_eq!(bulk, expected);
    let before = bulk.clone();
    assert!(state
        .extend_slices_into(&open, &high, &low, &close[..3], &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
