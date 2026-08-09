use super::triple_exponential_average::TripleExponentialAverage;
use super::StreamingIndicator;

fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
    (0..n)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
        })
        .collect()
}

fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
    assert_eq!(actual.len(), expected.len(), "{label}: length");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(actual.to_bits(), expected.to_bits(), "{label}: bar {index}");
    }
}

#[test]
fn bulk_is_bitwise_identical_to_per_bar_append() {
    let input = lcg_series(5_000, 0x5EED_7301);
    let tail = lcg_series(128, 0x7A11_7301);
    for (period, v_factor) in [(2usize, 0.7), (3, 0.0), (5, 1.0), (14, 0.7), (30, 0.2)] {
        let mut per_bar = TripleExponentialAverage::new(period, v_factor).unwrap();
        let reference: Vec<f64> = input
            .iter()
            .map(|&value| per_bar.append(value).unwrap_or(f64::NAN))
            .collect();
        let tail_reference: Vec<f64> = tail
            .iter()
            .map(|&value| per_bar.append(value).unwrap_or(f64::NAN))
            .collect();

        for chunk in [usize::MAX, 1, 7, 97] {
            let mut state = TripleExponentialAverage::new(period, v_factor).unwrap();
            let mut output = Vec::new();
            for piece in input.chunks(chunk.min(input.len())) {
                state.extend_slice_into(piece, &mut output);
            }
            assert_same_bits(&output, &reference, &format!("p{period} chunk {chunk}"));
            let tail_output: Vec<f64> = tail
                .iter()
                .map(|&value| state.append(value).unwrap_or(f64::NAN))
                .collect();
            assert_same_bits(
                &tail_output,
                &tail_reference,
                &format!("p{period} chunk {chunk} tail"),
            );
        }
    }
}

#[test]
fn reset_replays_the_same_state() {
    let input: Vec<f64> = (0..200)
        .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0 + index as f64 * 0.04)
        .collect();
    let mut state = TripleExponentialAverage::new(7, 0.7).unwrap();
    let mut first = Vec::new();
    state.extend_slice_into(&input, &mut first);
    let final_value = state.value();
    state.reset();
    let mut replay = Vec::new();
    state.extend_slice_into(&input, &mut replay);
    assert_same_bits(&first, &replay, "reset replay");
    assert_eq!(state.value(), final_value);
}

#[test]
fn rejects_periods_below_two() {
    assert!(TripleExponentialAverage::new(0, 0.7).is_err());
    assert!(TripleExponentialAverage::new(1, 0.7).is_err());
}
