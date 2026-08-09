use super::relative_momentum_index::RelativeMomentumIndex;
use super::StreamingIndicator;

#[test]
fn monotonic_input_warms_and_reaches_hundred() {
    let mut state = RelativeMomentumIndex::new(3, 2).unwrap();
    let values: Vec<_> = (1..=12).map(|value| state.append(value as f64)).collect();
    assert!(values[..4].iter().all(Option::is_none));
    assert!(values[4..]
        .iter()
        .all(|value| matches!(value, Some(v) if (*v - 100.0).abs() < 1e-12)));
}

#[test]
fn seed_is_an_average_before_wilder_continuation() {
    let mut state = RelativeMomentumIndex::new(2, 1).unwrap();
    let values = [100.0, 102.0, 101.0, 103.0];
    let actual: Vec<_> = values.iter().map(|&value| state.append(value)).collect();
    assert_eq!(actual[0], None);
    assert_eq!(actual[1], None);
    assert_eq!(actual[2], Some(66.66666666666667));
    assert_eq!(actual[3], Some(85.71428571428571));
}

#[test]
fn slice_extension_matches_scalar_replay_and_reset() {
    let inputs: Vec<_> = (0..257)
        .map(|index| 100.0 + (index as f64 * 0.17).sin())
        .collect();
    let mut batch = RelativeMomentumIndex::new(14, 5).unwrap();
    let mut expected = Vec::new();
    for &input in &inputs {
        expected.push(batch.append(input).unwrap_or(f64::NAN));
    }

    let mut chunked = RelativeMomentumIndex::new(14, 5).unwrap();
    let mut actual = Vec::new();
    chunked.extend_slice_into(&inputs[..31], &mut actual);
    chunked.extend_slice_into(&inputs[31..], &mut actual);
    assert!(actual
        .iter()
        .zip(&expected)
        .all(|(actual, expected)| actual.to_bits() == expected.to_bits()));

    chunked.reset();
    assert_eq!(chunked.value(), None);
    let mut replay = Vec::new();
    chunked.extend_slice_into(&inputs, &mut replay);
    assert!(replay
        .iter()
        .zip(&expected)
        .all(|(actual, expected)| actual.to_bits() == expected.to_bits()));
}

#[test]
fn rejects_non_positive_configuration() {
    assert!(RelativeMomentumIndex::new(0, 5).is_err());
    assert!(RelativeMomentumIndex::new(14, 0).is_err());
}
