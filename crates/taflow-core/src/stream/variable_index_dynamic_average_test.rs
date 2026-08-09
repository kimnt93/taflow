use super::{StreamingIndicator, VariableIndexDynamicAverage};

fn deterministic_series(length: usize) -> Vec<f64> {
    let mut seed = 0x71da_0000_0000_0002_u64;
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
fn variable_index_dynamic_average_matches_known_values() {
    let mut state = VariableIndexDynamicAverage::new(3, 0.5).unwrap();
    let actual = [1.0, 2.0, 3.0, 2.0, 4.0, 4.0].map(|input| state.append(input));
    assert_eq!(actual[0], None);
    assert_eq!(actual[1], None);
    assert_eq!(actual[2], Some(2.0));
    assert_eq!(actual[3], Some(2.0));
    assert_eq!(actual[4], Some(2.5));
    assert_eq!(actual[5], Some(2.75));
    assert_eq!(state.value(), Some(2.75));
}

#[test]
fn variable_index_dynamic_average_bulk_chunking_and_reset_are_bitwise_invariant() {
    let input = deterministic_series(5_000);
    for length in [1_usize, 2, 5, 14, 30] {
        let mut batch = VariableIndexDynamicAverage::new(length, 0.2).unwrap();
        let mut expected = Vec::new();
        batch.extend_slice_into(&input, &mut expected);

        for chunk_length in [1_usize, 7, 97, 5_000] {
            let mut chunked = VariableIndexDynamicAverage::new(length, 0.2).unwrap();
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

        batch.reset();
        assert_eq!(batch.value(), None);
        let mut replay = Vec::new();
        batch.extend_slice_into(&input, &mut replay);
        assert_same_bits(&replay, &expected);
    }
}

#[test]
fn variable_index_dynamic_average_validates_configuration() {
    assert!(VariableIndexDynamicAverage::new(0, 0.5).is_err());
    for invalid_alpha in [0.0, -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(VariableIndexDynamicAverage::new(14, invalid_alpha).is_err());
    }
}

#[test]
fn variable_index_dynamic_average_preserves_oracle_zero_momentum_semantics() {
    let mut state = VariableIndexDynamicAverage::new(3, 0.5).unwrap();
    assert_eq!(state.append(5.0), None);
    assert_eq!(state.append(5.0), None);
    assert_eq!(state.append(5.0), Some(5.0));
    assert!(state.append(5.0).unwrap().is_nan());
    assert!(state.append(6.0).unwrap().is_nan());
}
