use super::hilbert_transform_trendline::HilbertTransformTrendline;

#[test]
fn bulk_and_reset_replay_match() {
    let input: Vec<f64> = (0..256)
        .map(|index| 100.0 + (index as f64 * 0.15).sin())
        .collect();
    let mut state = HilbertTransformTrendline::new();
    let mut first = Vec::new();
    state.extend_slice_into(&input, &mut first);
    let final_value = state.value();
    state.reset();
    let mut second = Vec::new();
    state.extend_slice_into(&input, &mut second);
    assert_eq!(first.len(), second.len());
    for (first, second) in first.iter().zip(second) {
        assert_eq!(first.to_bits(), second.to_bits());
    }
    assert_eq!(state.value(), final_value);
}

#[test]
fn every_two_chunk_split_and_continuation_match_scalar_replay() {
    let input: Vec<f64> = (0..128)
        .map(|index| 100.0 + (index as f64 * 0.15).sin())
        .collect();
    let mut scalar = HilbertTransformTrendline::new();
    let expected: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    let expected_next = scalar.append(101.25);

    for split in 0..=input.len() {
        let mut chunked = HilbertTransformTrendline::new();
        let mut actual = Vec::new();
        chunked.extend_slice_into(&input[..split], &mut actual);
        chunked.extend_slice_into(&input[split..], &mut actual);
        assert_eq!(
            actual
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            "split {split}"
        );
        assert_eq!(
            chunked.append(101.25).map(f64::to_bits),
            expected_next.map(f64::to_bits),
            "continuation split {split}"
        );
    }
}
