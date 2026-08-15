use super::fast_stochastic_oscillator::FastStochasticOscillator;
use crate::ma_type::MaType;

fn aligned_scalar(
    state: &mut FastStochasticOscillator,
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> (Vec<f64>, Vec<f64>) {
    let mut fastk = Vec::with_capacity(high.len());
    let mut fastd = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        match state.append(high, low, close) {
            Some(value) => {
                fastk.push(value.fastk);
                fastd.push(value.fastd);
            }
            None => {
                fastk.push(f64::NAN);
                fastd.push(f64::NAN);
            }
        }
    }
    (fastk, fastd)
}

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn scalar_bulk_all_splits_continuation_and_reset_are_bitwise_invariant() {
    let high: Vec<f64> = (0..97)
        .map(|i| 100.0 + i as f64 * 0.2 + (i as f64 * 0.71).sin())
        .collect();
    let low: Vec<f64> = high
        .iter()
        .enumerate()
        .map(|(index, high)| high - 1.5 - (index as f64 * 0.31).cos().abs())
        .collect();
    let close: Vec<f64> = high
        .iter()
        .zip(&low)
        .enumerate()
        .map(|(index, (high, low))| low + (high - low) * ((index % 7) as f64 / 6.0))
        .collect();

    for average_type in [
        MaType::SimpleMovingAverage,
        MaType::ExponentialMovingAverage,
    ] {
        let mut scalar = FastStochasticOscillator::new(5, 3, average_type).unwrap();
        let (expected_k, expected_d) = aligned_scalar(&mut scalar, &high, &low, &close);
        let expected_value = scalar.value();
        let expected_next = scalar.append(110.0, 107.0, 108.5);

        for split in 0..=high.len() {
            let mut bulk = FastStochasticOscillator::new(5, 3, average_type).unwrap();
            let (mut actual_k, mut actual_d) = (Vec::new(), Vec::new());
            bulk.extend_slices_into(
                &high[..split],
                &low[..split],
                &close[..split],
                &mut actual_k,
                &mut actual_d,
            )
            .unwrap();
            bulk.extend_slices_into(
                &high[split..],
                &low[split..],
                &close[split..],
                &mut actual_k,
                &mut actual_d,
            )
            .unwrap();
            assert_bits_equal(&actual_k, &expected_k);
            assert_bits_equal(&actual_d, &expected_d);
            assert_eq!(bulk.value(), expected_value);
            assert_eq!(bulk.append(110.0, 107.0, 108.5), expected_next);
        }

        let mut replay = FastStochasticOscillator::new(5, 3, average_type).unwrap();
        let (mut replay_k, mut replay_d) = (Vec::new(), Vec::new());
        replay
            .extend_slices_into(&high, &low, &close, &mut replay_k, &mut replay_d)
            .unwrap();
        replay.reset();
        replay_k.clear();
        replay_d.clear();
        replay
            .extend_slices_into(&high, &low, &close, &mut replay_k, &mut replay_d)
            .unwrap();
        assert_bits_equal(&replay_k, &expected_k);
        assert_bits_equal(&replay_d, &expected_d);

        let before_k = replay_k.clone();
        let before_d = replay_d.clone();
        let before_value = replay.value();
        assert!(replay
            .extend_slices_into(
                &high,
                &low[..low.len() - 1],
                &close,
                &mut replay_k,
                &mut replay_d,
            )
            .is_err());
        assert_bits_equal(&replay_k, &before_k);
        assert_bits_equal(&replay_d, &before_d);
        assert_eq!(replay.value(), before_value);
    }
}
