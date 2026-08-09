use super::accumulation_distribution::AccumulationDistribution;

#[test]
fn lifecycle_bulk_and_validation_are_bitwise_consistent() {
    let high = [12.0_f64, 14.0, 9.0, 7.0];
    let low = [8.0_f64, 10.0, 9.0, 5.0];
    let close = [11.0_f64, 11.0, 9.0, 6.5];
    let volume = [100.0_f64, 200.0, 300.0, 400.0];
    let expected = [50.0_f64, -50.0, -50.0, 150.0];

    let mut scalar_state = AccumulationDistribution::new().unwrap();
    assert_eq!(scalar_state.value(), None);
    let scalar: Vec<_> = (0..high.len())
        .map(|index| scalar_state.append(high[index], low[index], close[index], volume[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(scalar_state.value(), Some(150.0));

    scalar_state.reset();
    let replay: Vec<_> = (0..high.len())
        .map(|index| scalar_state.append(high[index], low[index], close[index], volume[index]))
        .collect();
    assert_eq!(replay, expected);

    let mut bulk_state = AccumulationDistribution::new().unwrap();
    let mut bulk = Vec::new();
    bulk_state
        .extend_slices_into(&high[..2], &low[..2], &close[..2], &volume[..2], &mut bulk)
        .unwrap();
    bulk_state
        .extend_slices_into(&high[2..], &low[2..], &close[2..], &volume[2..], &mut bulk)
        .unwrap();
    assert_eq!(bulk, expected);
    assert_eq!(bulk_state.value(), scalar_state.value());

    let before = bulk.clone();
    assert!(bulk_state
        .extend_slices_into(&high, &low, &close[..3], &volume, &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
