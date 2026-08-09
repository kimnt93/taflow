use super::median_price::MedianPrice;

#[test]
fn lifecycle_bulk_and_validation_are_consistent() {
    let high = [12.0_f64, 13.0, 14.0, 15.0];
    let low = [8.0_f64, 9.0, 10.0, 11.0];
    let expected: Vec<_> = (0..high.len())
        .map(|index| {
            let high = high[index];
            let low = low[index];
            (high + low) * 0.5
        })
        .collect();
    let mut state = MedianPrice::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = (0..high.len())
        .map(|index| state.append(high[index], low[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(state.value(), expected.last().copied());
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slices_into(&high, &low, &mut bulk).unwrap();
    assert_eq!(bulk, expected);
    let before = bulk.clone();
    assert!(state
        .extend_slices_into(&high, &low[..3], &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
