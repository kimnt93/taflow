use super::aroon::{Aroon, AroonValue};

fn datasets(length: usize) -> Vec<(Vec<f64>, Vec<f64>)> {
    let wave: Vec<_> = (0..length)
        .map(|index| (index as f64 * 0.17).sin() * 20.0 + index as f64 * 0.01)
        .collect();
    let ties: Vec<_> = (0..length).map(|index| ((index * 7) % 5) as f64).collect();
    let constant = vec![13.25_f64; length];
    let increasing: Vec<_> = (0..length).map(|index| index as f64).collect();
    let decreasing: Vec<_> = increasing.iter().rev().copied().collect();
    vec![
        (wave.clone(), wave),
        (ties.clone(), ties),
        (constant.clone(), constant),
        (increasing.clone(), increasing),
        (decreasing.clone(), decreasing),
    ]
}

#[test]
fn scalar_bulk_reset_and_continuation_are_bitwise_identical() {
    for (high, low) in datasets(1_003) {
        for period in [2_usize, 5, 14, 30, 200] {
            let mut scalar_state = Aroon::new(period).unwrap();
            let scalar: Vec<_> = (0..high.len())
                .map(|index| {
                    scalar_state
                        .append(high[index], low[index])
                        .unwrap_or(AroonValue {
                            down: f64::NAN,
                            up: f64::NAN,
                        })
                })
                .collect();

            for chunk in [1_usize, 7, 97, high.len()] {
                let mut bulk_state = Aroon::new(period).unwrap();
                let (mut down, mut up) = (Vec::new(), Vec::new());
                let mut offset = 0;
                while offset < high.len() {
                    let end = (offset + chunk).min(high.len());
                    bulk_state
                        .extend_slices_into(
                            &high[offset..end],
                            &low[offset..end],
                            &mut down,
                            &mut up,
                        )
                        .unwrap();
                    offset = end;
                }
                for (index, expected) in scalar.iter().enumerate() {
                    assert_eq!(down[index].to_bits(), expected.down.to_bits());
                    assert_eq!(up[index].to_bits(), expected.up.to_bits());
                }
                assert_eq!(bulk_state.value(), scalar_state.value());
                let mut continued = scalar_state.clone();
                assert_eq!(bulk_state.append(21.0, -3.0), continued.append(21.0, -3.0));
            }

            scalar_state.reset();
            assert_eq!(scalar_state.value(), None);
            for index in 0..high.len() {
                assert_eq!(scalar_state.append(high[index], low[index]), {
                    let expected = scalar[index];
                    if expected.down.is_nan() {
                        None
                    } else {
                        Some(expected)
                    }
                });
            }
        }
    }
}

#[test]
fn configuration_and_lengths_are_validated_before_mutation() {
    assert!(Aroon::new(1).is_err());
    let mut state = Aroon::new(5).unwrap();
    let (mut down, mut up) = (Vec::new(), Vec::new());
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &mut down, &mut up)
        .is_err());
    assert!(down.is_empty());
    assert!(up.is_empty());
    assert_eq!(state.value(), None);
}
