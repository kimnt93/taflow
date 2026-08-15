use super::rolling_mode::RollingMode;

fn reference_mode(window: &[f64]) -> f64 {
    let mut best = window[0];
    let mut best_count = 0;
    for &candidate in window {
        let count = window.iter().filter(|&&value| value == candidate).count();
        if count > best_count {
            best = candidate;
            best_count = count;
        }
    }
    best
}

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingMode::new(3).unwrap();
    for value in [4.0, 2.0, 2.0, 1.0, 5.0] {
        state.append(value);
    }
    let expected = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for value in [4.0, 2.0, 2.0, 1.0, 5.0] {
        state.append(value);
    }
    assert_eq!(state.value(), expected);
}

#[test]
fn indexed_heap_matches_window_order_ties() {
    let period = 19;
    let values = (0..4_000)
        .map(|index| ((index * 29 + index / 5) % 17) as f64 - 8.0)
        .collect::<Vec<_>>();
    let mut state = RollingMode::new(period).unwrap();

    for (index, &value) in values.iter().enumerate() {
        let actual = state.append(value);
        if index + 1 < period {
            assert_eq!(actual, None);
        } else {
            let expected = reference_mode(&values[index + 1 - period..=index]);
            assert_eq!(actual.unwrap().to_bits(), expected.to_bits());
        }
    }
}

#[test]
fn nan_and_signed_zero_follow_original_equality_rules() {
    let mut state = RollingMode::new(4).unwrap();
    for value in [f64::NAN, -0.0, 0.0] {
        assert_eq!(state.append(value), None);
    }
    assert_eq!(state.append(2.0).unwrap().to_bits(), (-0.0f64).to_bits());
    assert_eq!(state.append(2.0).unwrap().to_bits(), (-0.0f64).to_bits());
    assert_eq!(state.append(f64::NAN), Some(2.0));

    let mut all_nan = RollingMode::new(3).unwrap();
    assert_eq!(all_nan.append(f64::NAN), None);
    assert_eq!(all_nan.append(f64::NAN), None);
    assert!(all_nan.append(f64::NAN).unwrap().is_nan());
}

#[test]
fn bulk_chunks_and_reset_replay_match_scalar_bits() {
    let values = (0..521)
        .map(|index| ((index * 31 + index / 9) % 29) as f64)
        .collect::<Vec<_>>();
    let mut scalar = RollingMode::new(23).unwrap();
    let expected = values
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN).to_bits())
        .collect::<Vec<_>>();

    let mut chunked = RollingMode::new(23).unwrap();
    let mut actual = Vec::new();
    chunked.extend_slice_into(&values[..101], &mut actual);
    chunked.extend_slice_into(&values[101..333], &mut actual);
    chunked.extend_slice_into(&values[333..], &mut actual);
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(
        chunked.value().unwrap().to_bits(),
        scalar.value().unwrap().to_bits()
    );

    chunked.reset();
    let mut replay = Vec::new();
    chunked.extend_slice_into(&values, &mut replay);
    assert_eq!(
        replay
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
    );
}
