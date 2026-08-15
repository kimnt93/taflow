use super::rolling_entropy::RollingEntropy;

fn reference_entropy(window: &[f64]) -> f64 {
    let n = window.len() as f64;
    let mut entropy = 0.0;
    for (index, &candidate) in window.iter().enumerate() {
        if window[..index].contains(&candidate) {
            continue;
        }
        let probability = if candidate.is_nan() {
            0.0
        } else {
            window.iter().filter(|&&value| value == candidate).count() as f64 / n
        };
        entropy -= probability * probability.ln();
    }
    entropy
}

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = RollingEntropy::new(2).unwrap();
    assert_eq!(state.append(1.0), None);
    assert!(state.append(2.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn incremental_counts_match_direct_frequency_entropy() {
    let period = 17;
    let values = (0..2_000)
        .map(|index| ((index * 37 + index / 11) % 13) as f64 - 6.0)
        .collect::<Vec<_>>();
    let mut state = RollingEntropy::new(period).unwrap();

    for (index, &value) in values.iter().enumerate() {
        let actual = state.append(value);
        if index + 1 < period {
            assert_eq!(actual, None);
        } else {
            let expected = reference_entropy(&values[index + 1 - period..=index]);
            assert!((actual.unwrap() - expected).abs() <= 2.0e-14);
        }
    }
}

#[test]
fn nan_and_signed_zero_semantics_are_preserved() {
    let mut state = RollingEntropy::new(3).unwrap();
    assert_eq!(state.append(-0.0), None);
    assert_eq!(state.append(0.0), None);
    let expected = reference_entropy(&[-0.0, 0.0, 1.0]);
    assert!((state.append(1.0).unwrap() - expected).abs() <= f64::EPSILON);
    assert!(state.append(f64::NAN).unwrap().is_nan());
    assert!(state.append(2.0).unwrap().is_nan());
    assert!(state.append(3.0).unwrap().is_nan());
    assert!(state.append(4.0).unwrap().is_finite());
}

#[test]
fn bulk_chunks_and_reset_replay_match_scalar_bits() {
    let values = (0..257)
        .map(|index| ((index * 19 + index / 7) % 23) as f64)
        .collect::<Vec<_>>();
    let mut scalar = RollingEntropy::new(11).unwrap();
    let expected = values
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN).to_bits())
        .collect::<Vec<_>>();

    let mut chunked = RollingEntropy::new(11).unwrap();
    let mut actual = Vec::new();
    chunked.extend_slice_into(&values[..73], &mut actual);
    chunked.extend_slice_into(&values[73..], &mut actual);
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
