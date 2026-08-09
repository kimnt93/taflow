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
