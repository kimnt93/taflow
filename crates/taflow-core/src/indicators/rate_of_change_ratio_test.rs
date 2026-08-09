use super::rate_of_change_ratio::RateOfChangeRatio;

#[test]
fn rate_of_change_ratio_matches_expected_and_preserves_lifecycle() {
    let input = [10.0, 12.0, 0.0, 18.0, 9.0, 27.0];
    let expected = [f64::NAN, f64::NAN, 0.0, 1.5, 0.0, 1.5];
    let mut state = RateOfChangeRatio::new(2).unwrap();
    let mut actual = Vec::new();
    state.extend_slice_into(&input[..3], &mut actual);
    state.extend_slice_into(&input[3..], &mut actual);
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected.map(f64::to_bits)
    );
    assert_eq!(state.value(), Some(1.5));

    state.reset();
    let replay = input.map(|value| state.append(value).unwrap_or(f64::NAN));
    assert_eq!(replay.map(f64::to_bits), expected.map(f64::to_bits));
    assert!(RateOfChangeRatio::new(0).is_err());
}
