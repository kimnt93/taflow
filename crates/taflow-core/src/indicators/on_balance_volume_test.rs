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

#[test]
fn bulk_all_splits_and_reset_are_bitwise_scalar_equivalent() {
    let close: Vec<_> = (0..129)
        .map(|index| 100.0 + (index as f64 * 0.31).sin())
        .collect();
    let volume: Vec<_> = (0..129)
        .map(|index| 10.0 + ((index * 17) % 31) as f64)
        .collect();
    let mut scalar = OnBalanceVolume::new().unwrap();
    let expected: Vec<_> = close
        .iter()
        .zip(&volume)
        .map(|(&close, &volume)| scalar.append(close, volume))
        .collect();
    let scalar_value = scalar.value();

    for split in 0..=close.len() {
        let mut state = OnBalanceVolume::new().unwrap();
        let mut actual = Vec::new();
        state
            .extend_slices_into(&close[..split], &volume[..split], &mut actual)
            .unwrap();
        state
            .extend_slices_into(&close[split..], &volume[split..], &mut actual)
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
        assert_eq!(
            state.value().map(f64::to_bits),
            scalar_value.map(f64::to_bits)
        );
    }

    scalar.reset();
    let mut replay = Vec::new();
    scalar
        .extend_slices_into(&close, &volume, &mut replay)
        .unwrap();
    assert_eq!(
        replay
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
}
