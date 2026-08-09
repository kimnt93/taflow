use super::moving_average_convergence_divergence::MovingAverageConvergenceDivergence;

fn assert_same_bits(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn bulk_and_scalar_replay_are_bitwise_identical() {
    let input: Vec<f64> = (0..512)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 4.0)
        .collect();
    let mut scalar = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
    let mut expected = (Vec::new(), Vec::new(), Vec::new());
    for value in input.iter().copied() {
        if let Some(value) = scalar.append(value) {
            expected.0.push(value.macd);
            expected.1.push(value.signal);
            expected.2.push(value.histogram);
        } else {
            expected.0.push(f64::NAN);
            expected.1.push(f64::NAN);
            expected.2.push(f64::NAN);
        }
    }
    let mut bulk = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
    let mut actual = (Vec::new(), Vec::new(), Vec::new());
    for chunk in input.chunks(37) {
        bulk.extend_slices_into(chunk, &mut actual.0, &mut actual.1, &mut actual.2);
    }
    assert_same_bits(&actual.0, &expected.0);
    assert_same_bits(&actual.1, &expected.1);
    assert_same_bits(&actual.2, &expected.2);
    let final_value = bulk.value();
    bulk.reset();
    for value in input {
        bulk.append(value);
    }
    assert_eq!(bulk.value(), final_value);
}
