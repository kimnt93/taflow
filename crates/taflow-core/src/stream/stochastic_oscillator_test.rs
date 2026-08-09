use super::stochastic_oscillator::StochasticOscillator;
use crate::ma_type::MaType;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let high: Vec<f64> = (0..96)
        .map(|i| 100.0 + i as f64 * 0.2 + (i as f64).sin())
        .collect();
    let low: Vec<f64> = high.iter().map(|x| x - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|x| x - 0.7).collect();
    let mut scalar = StochasticOscillator::new(
        5,
        3,
        MaType::SimpleMovingAverage,
        3,
        MaType::SimpleMovingAverage,
    )
    .unwrap();
    let scalar_out: Vec<_> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| scalar.append(h, l, c))
        .collect();
    let mut bulk = StochasticOscillator::new(
        5,
        3,
        MaType::SimpleMovingAverage,
        3,
        MaType::SimpleMovingAverage,
    )
    .unwrap();
    let (mut slowk, mut slowd) = (Vec::new(), Vec::new());
    bulk.extend_slices_into(&high, &low, &close, &mut slowk, &mut slowd)
        .unwrap();
    for (i, value) in scalar_out.iter().enumerate() {
        match value {
            Some(v) => {
                assert_eq!(v.slowk.to_bits(), slowk[i].to_bits());
                assert_eq!(v.slowd.to_bits(), slowd[i].to_bits());
            }
            None => assert!(slowk[i].is_nan() && slowd[i].is_nan()),
        }
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
