use super::{
    bollinger_bands::{BollingerBands, BollingerBandsValue},
    StreamingIndicator,
};
use crate::ma_type::MaType;

fn assert_value_bits_equal(left: BollingerBandsValue, right: BollingerBandsValue) {
    for (actual, expected) in [
        (left.upper, right.upper),
        (left.middle, right.middle),
        (left.lower, right.lower),
    ] {
        assert!(
            actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan()),
            "{actual:?} != {expected:?}"
        );
    }
}

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

#[test]
fn every_moving_average_type_is_bulk_chunk_and_continuation_invariant() {
    let input: Vec<f64> = (0..192)
        .map(|i| 80.0 + (i as f64 * 0.173).sin() * 3.0 + i as f64 * 0.021)
        .collect();
    for code in 0..=8 {
        let ma_type = MaType::try_from(code).unwrap();
        let mut scalar = BollingerBands::new(10, 2.3, 1.7, ma_type).unwrap();
        let expected: Vec<_> = input
            .iter()
            .map(|&value| {
                scalar.append(value).unwrap_or(BollingerBandsValue {
                    upper: f64::NAN,
                    middle: f64::NAN,
                    lower: f64::NAN,
                })
            })
            .collect();
        let expected_value = scalar.value();
        let expected_continuation = scalar.append(91.25);

        for splits in [&[192][..], &[1, 7, 13, 41, 130][..]] {
            let mut bulk = BollingerBands::new(10, 2.3, 1.7, ma_type).unwrap();
            let (mut upper, mut middle, mut lower) = (Vec::new(), Vec::new(), Vec::new());
            let mut offset = 0;
            for &size in splits {
                bulk.extend_slices_into(
                    &input[offset..offset + size],
                    &mut upper,
                    &mut middle,
                    &mut lower,
                );
                offset += size;
            }
            for (index, expected) in expected.iter().copied().enumerate() {
                assert_value_bits_equal(
                    BollingerBandsValue {
                        upper: upper[index],
                        middle: middle[index],
                        lower: lower[index],
                    },
                    expected,
                );
            }
            assert_eq!(bulk.value(), expected_value, "matype {code}");
            assert_eq!(
                bulk.append(91.25),
                expected_continuation,
                "continuation matype {code}"
            );
        }
    }
}
