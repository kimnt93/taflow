use super::balance_of_power::BalanceOfPower;

#[test]
fn lifecycle_bulk_and_validation_are_consistent() {
    let open = [10.0_f64, 12.0, 8.0, 7.0];
    let high = [14.0_f64, 12.0, 12.0, 9.0];
    let low = [8.0_f64, 12.0, 8.0, 5.0];
    let close = [13.0_f64, 12.0, 10.0, 6.0];
    let expected = [0.5_f64, 0.0, 0.5, -0.25];

    let mut scalar_state = BalanceOfPower::new().unwrap();
    assert_eq!(scalar_state.value(), None);
    let scalar: Vec<_> = (0..open.len())
        .map(|index| scalar_state.append(open[index], high[index], low[index], close[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(scalar_state.value(), expected.last().copied());

    scalar_state.reset();
    assert_eq!(scalar_state.value(), None);
    let replay: Vec<_> = (0..open.len())
        .map(|index| scalar_state.append(open[index], high[index], low[index], close[index]))
        .collect();
    assert_eq!(replay, expected);

    let mut bulk_state = BalanceOfPower::new().unwrap();
    let mut bulk = Vec::new();
    bulk_state
        .extend_slices_into(&open[..2], &high[..2], &low[..2], &close[..2], &mut bulk)
        .unwrap();
    bulk_state
        .extend_slices_into(&open[2..], &high[2..], &low[2..], &close[2..], &mut bulk)
        .unwrap();
    assert_eq!(bulk, expected);
    assert_eq!(bulk_state.value(), scalar_state.value());

    let before = bulk.clone();
    assert!(bulk_state
        .extend_slices_into(&open, &high[..3], &low, &close, &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
