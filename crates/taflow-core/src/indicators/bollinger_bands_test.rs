use super::{
    bollinger_bands::{BollingerBands, BollingerBandsValue},
    StreamingIndicator,
};
use crate::ma_type::MaType;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..96).map(|i| 100.0 + (i as f64 * 0.31).sin()).collect();
    let mut scalar = BollingerBands::new(10, 2.0, 2.0, MaType::SimpleMovingAverage).unwrap();
    let scalar_out: Vec<BollingerBandsValue> = input
        .iter()
        .map(|&x| {
            scalar.append(x).unwrap_or(BollingerBandsValue {
                upper: f64::NAN,
                middle: f64::NAN,
                lower: f64::NAN,
            })
        })
        .collect();
    let mut bulk = BollingerBands::new(10, 2.0, 2.0, MaType::SimpleMovingAverage).unwrap();
    let (mut upper, mut middle, mut lower) = (Vec::new(), Vec::new(), Vec::new());
    bulk.extend_slices_into(&input, &mut upper, &mut middle, &mut lower);
    for (i, value) in scalar_out.iter().enumerate() {
        for (a, b) in [
            (value.upper, upper[i]),
            (value.middle, middle[i]),
            (value.lower, lower[i]),
        ] {
            assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
        }
    }
    assert_eq!(
        scalar
            .value()
            .map(|v| (v.upper.to_bits(), v.middle.to_bits(), v.lower.to_bits())),
        bulk.value()
            .map(|v| (v.upper.to_bits(), v.middle.to_bits(), v.lower.to_bits()))
    );
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
