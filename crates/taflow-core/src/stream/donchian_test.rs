use super::donchian::Donchian;

#[test]
fn scalar_bulk_and_reset_are_consistent() {
    let high = [10.0, 12.0, 11.0, 13.0];
    let low = [8.0, 9.0, 7.0, 10.0];
    let mut state = Donchian::new(3).unwrap();
    assert_eq!(state.append(high[0], low[0]), None);
    assert_eq!(state.append(high[1], low[1]), None);
    let value = state.append(high[2], low[2]).unwrap();
    assert_eq!(value.upper, 12.0);
    assert_eq!(value.lower, 7.0);
    assert_eq!(value.middle, 9.5);
    state.reset();
    assert_eq!(state.value(), None);
    let mut bulk = Donchian::new(3).unwrap();
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    let mut middle = Vec::new();
    bulk.extend_slices_into(&high, &low, &mut upper, &mut lower, &mut middle)
        .unwrap();
    assert!(upper[0].is_nan() && upper[1].is_nan());
    assert_eq!(upper[2].to_bits(), 12.0f64.to_bits());
    assert_eq!(lower[3].to_bits(), 7.0f64.to_bits());
}
