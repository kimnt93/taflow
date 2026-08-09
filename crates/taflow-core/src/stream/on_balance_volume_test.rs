use super::on_balance_volume::OnBalanceVolume;

#[test]
fn lifecycle_bulk_validation_and_continuation_are_bitwise_consistent() {
    let close = [10.0_f64, 11.0, 11.0, 9.0, 12.0];
    let volume = [100.0_f64, 20.0, 30.0, 40.0, 50.0];
    let expected = [100.0_f64, 120.0, 120.0, 80.0, 130.0];

    let mut scalar_state = OnBalanceVolume::new().unwrap();
    assert_eq!(scalar_state.value(), None);
    let scalar: Vec<_> = (0..close.len())
        .map(|index| scalar_state.append(close[index], volume[index]))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(scalar_state.value(), Some(130.0));

    scalar_state.reset();
    assert_eq!(scalar_state.value(), None);
    let replay: Vec<_> = (0..close.len())
        .map(|index| scalar_state.append(close[index], volume[index]))
        .collect();
    assert_eq!(replay, expected);

    let mut bulk_state = OnBalanceVolume::new().unwrap();
    let mut bulk = Vec::new();
    bulk_state
        .extend_slices_into(&close[..2], &volume[..2], &mut bulk)
        .unwrap();
    bulk_state
        .extend_slices_into(&close[2..], &volume[2..], &mut bulk)
        .unwrap();
    assert_eq!(bulk, expected);
    assert_eq!(bulk_state.value(), scalar_state.value());
    assert_eq!(bulk_state.append(13.0, 60.0).to_bits(), 190.0_f64.to_bits());
    assert_eq!(
        scalar_state.append(13.0, 60.0).to_bits(),
        190.0_f64.to_bits()
    );

    let before = bulk.clone();
    assert!(bulk_state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &mut bulk)
        .is_err());
    assert_eq!(bulk, before);
}
