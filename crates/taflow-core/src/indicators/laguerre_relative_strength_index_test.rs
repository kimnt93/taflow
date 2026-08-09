use super::{LaguerreRelativeStrengthIndex, StreamingIndicator};

fn deterministic_series(length: usize) -> Vec<f64> {
    let mut seed = 0x1a6a_e22e_0000_0001_u64;
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
fn laguerre_relative_strength_index_matches_known_values() {
    let mut state = LaguerreRelativeStrengthIndex::new(0.5).unwrap();
    let actual = [1.0, 2.0, 3.0, 2.0, 4.0, 4.0].map(|input| state.append(input).unwrap());
    let expected = [
        0.0,
        71.42857142857143,
        80.0,
        66.66666666666667,
        100.0,
        100.0,
    ];
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12);
    }
    assert_eq!(state.value(), Some(100.0));
}

#[test]
fn laguerre_relative_strength_index_bulk_chunking_and_reset_are_bitwise_invariant() {
    let input = deterministic_series(5_000);
    for gamma in [0.0, 0.1, 0.5, 0.9, 0.999] {
        let mut batch = LaguerreRelativeStrengthIndex::new(gamma).unwrap();
        let mut expected = Vec::new();
        batch.extend_slice_into(&input, &mut expected);

        for chunk_length in [1_usize, 7, 97, 5_000] {
            let mut chunked = LaguerreRelativeStrengthIndex::new(gamma).unwrap();
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
fn laguerre_relative_strength_index_handles_constant_and_single_input_series() {
    let mut state = LaguerreRelativeStrengthIndex::new(0.5).unwrap();
    for _ in 0..32 {
        assert_eq!(state.append(42.0), Some(0.0));
    }
}

#[test]
fn laguerre_relative_strength_index_validates_gamma() {
    for invalid in [-0.1, 1.0, 1.1, f64::NAN, f64::INFINITY] {
        assert!(LaguerreRelativeStrengthIndex::new(invalid).is_err());
    }
    assert!(LaguerreRelativeStrengthIndex::new(0.0).is_ok());
}
