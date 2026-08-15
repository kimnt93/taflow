use super::{RelativeStrengthIndex, StreamingIndicator};

fn deterministic_series(length: usize) -> Vec<f64> {
    let mut seed = 0x2515_0000_0000_0003_u64;
    (0..length)
        .map(|_| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            100.0 + ((seed >> 11) as f64 / (1_u64 << 53) as f64 - 0.5) * 20.0
        })
        .collect()
}

fn assert_same_bits(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len());
    for (&left, &right) in left.iter().zip(right) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
}

#[test]
fn relative_strength_index_matches_known_wilder_values() {
    let input = [1.0, 2.0, 3.0, 2.0, 4.0, 4.0];
    let mut state = RelativeStrengthIndex::new(3).unwrap();
    let actual = input.map(|value| state.append(value));
    assert_eq!(actual[0], None);
    assert_eq!(actual[1], None);
    assert_eq!(actual[2], None);
    for (actual, expected) in
        actual[3..]
            .iter()
            .zip([66.66666666666666, 83.33333333333333, 83.33333333333333])
    {
        assert!((actual.unwrap() - expected).abs() < 1e-12);
    }
}

#[test]
fn relative_strength_index_bulk_chunking_and_reset_are_bitwise_invariant() {
    let input = deterministic_series(5_000);
    for period in [2_usize, 3, 14, 30] {
        let mut batch = RelativeStrengthIndex::new(period).unwrap();
        let mut expected = Vec::new();
        batch.extend_slice_into(&input, &mut expected);

        for chunk_length in [1_usize, 7, 97, 5_000] {
            let mut chunked = RelativeStrengthIndex::new(period).unwrap();
            let mut actual = Vec::new();
            for chunk in input.chunks(chunk_length) {
                chunked.extend_slice_into(chunk, &mut actual);
            }
            assert_same_bits(&actual, &expected);
            assert_eq!(
                chunked.value().map(f64::to_bits),
                batch.value().map(f64::to_bits)
            );
        }

        let split_input = &input[..257];
        let mut split_batch = RelativeStrengthIndex::new(period).unwrap();
        let mut split_expected = Vec::new();
        split_batch.extend_slice_into(split_input, &mut split_expected);
        let continuation = split_batch.append(103.25).map(f64::to_bits);
        for split in 0..=split_input.len() {
            let mut chunked = RelativeStrengthIndex::new(period).unwrap();
            let mut actual = Vec::new();
            chunked.extend_slice_into(&split_input[..split], &mut actual);
            chunked.extend_slice_into(&split_input[split..], &mut actual);
            assert_same_bits(&actual, &split_expected);
            assert_eq!(chunked.append(103.25).map(f64::to_bits), continuation);
        }

        batch.reset();
        assert_eq!(batch.value(), None);
        let mut replay = Vec::new();
        batch.extend_slice_into(&input, &mut replay);
        assert_same_bits(&replay, &expected);
    }
}

#[test]
fn relative_strength_index_preserves_constant_series_semantics() {
    let mut state = RelativeStrengthIndex::new(3).unwrap();
    assert_eq!(state.append(42.0), None);
    assert_eq!(state.append(42.0), None);
    assert_eq!(state.append(42.0), None);
    assert_eq!(state.append(42.0), Some(0.0));
}

#[test]
fn relative_strength_index_validates_period() {
    assert!(RelativeStrengthIndex::new(0).is_err());
    assert!(RelativeStrengthIndex::new(1).is_err());
    assert!(RelativeStrengthIndex::new(2).is_ok());
}
