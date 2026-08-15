use super::stochastic_relative_strength_index::StochasticRelativeStrengthIndex;
use crate::ma_type::MaType;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..160)
        .map(|i| 100.0 + (i as f64 * 0.23).sin() + i as f64 * 0.01)
        .collect();
    let mut scalar =
        StochasticRelativeStrengthIndex::new(14, 5, 3, MaType::SimpleMovingAverage).unwrap();
    let scalar_out: Vec<_> = input.iter().map(|&x| scalar.append(x)).collect();
    let mut bulk =
        StochasticRelativeStrengthIndex::new(14, 5, 3, MaType::SimpleMovingAverage).unwrap();
    let (mut fastk, mut fastd) = (Vec::new(), Vec::new());
    bulk.extend_slices_into(&input, &mut fastk, &mut fastd);
    for (i, value) in scalar_out.iter().enumerate() {
        match value {
            Some(v) => {
                assert_eq!(v.fastk.to_bits(), fastk[i].to_bits());
                assert_eq!(v.fastd.to_bits(), fastd[i].to_bits());
            }
            None => assert!(fastk[i].is_nan() && fastd[i].is_nan()),
        }
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}

#[test]
fn every_two_chunk_split_and_continuation_match_scalar_replay() {
    let input: Vec<f64> = (0..96)
        .map(|i| 100.0 + (i as f64 * 0.23).sin() + i as f64 * 0.01)
        .collect();
    let mut scalar =
        StochasticRelativeStrengthIndex::new(14, 5, 3, MaType::SimpleMovingAverage).unwrap();
    let expected: Vec<_> = input.iter().map(|&value| scalar.append(value)).collect();
    let expected_continuation = scalar.append(103.75);

    for split in 0..=input.len() {
        let mut chunked =
            StochasticRelativeStrengthIndex::new(14, 5, 3, MaType::SimpleMovingAverage).unwrap();
        let (mut fastk, mut fastd) = (Vec::new(), Vec::new());
        chunked.extend_slices_into(&input[..split], &mut fastk, &mut fastd);
        chunked.extend_slices_into(&input[split..], &mut fastk, &mut fastd);
        for (index, value) in expected.iter().enumerate() {
            match value {
                Some(value) => {
                    assert_eq!(fastk[index].to_bits(), value.fastk.to_bits());
                    assert_eq!(fastd[index].to_bits(), value.fastd.to_bits());
                }
                None => assert!(fastk[index].is_nan() && fastd[index].is_nan()),
            }
        }

        let continuation = chunked.append(103.75);
        assert_eq!(continuation, expected_continuation, "split {split}");
    }
}
