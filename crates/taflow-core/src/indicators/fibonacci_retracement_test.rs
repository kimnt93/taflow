use super::fibonacci_retracement::{FibonacciRetracement, FibonacciRetracementValue};

fn values(value: FibonacciRetracementValue) -> [f64; 7] {
    [
        value.level_zero,
        value.level_twenty_three_point_six,
        value.level_thirty_eight_point_two,
        value.level_fifty,
        value.level_sixty_one_point_eight,
        value.level_seventy_eight_point_six,
        value.level_one_hundred,
    ]
}

fn lcg_series(length: usize, mut state: u64) -> Vec<f64> {
    (0..length)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            90.0 + (state >> 11) as f64 / (1_u64 << 53) as f64 * 20.0
        })
        .collect()
}

#[test]
fn scalar_bulk_chunking_and_reset_are_bitwise_identical() {
    let mut input = lcg_series(2_000, 0xF1B0_0002);
    for index in (0..input.len()).step_by(23) {
        input[index] = f64::NAN;
    }

    let mut scalar = FibonacciRetracement::new(120).unwrap();
    let expected: Vec<_> = input
        .iter()
        .map(|&close| values(scalar.append(close)))
        .collect();

    let mut bulk = FibonacciRetracement::new(120).unwrap();
    let mut outputs: [Vec<f64>; 7] = std::array::from_fn(|_| Vec::new());
    let [zero, twenty_three, thirty_eight, fifty, sixty_one, seventy_eight, hundred] = &mut outputs;
    bulk.extend_slice_into(
        &input,
        zero,
        twenty_three,
        thirty_eight,
        fifty,
        sixty_one,
        seventy_eight,
        hundred,
    );
    for (index, expected) in expected.iter().enumerate() {
        for level in 0..7 {
            assert_eq!(outputs[level][index].to_bits(), expected[level].to_bits());
        }
    }
    assert_eq!(bulk.value(), scalar.value());

    let mut chunked = FibonacciRetracement::new(120).unwrap();
    let mut chunked_outputs: [Vec<f64>; 7] = std::array::from_fn(|_| Vec::new());
    for chunk in input.chunks(37) {
        let [zero, twenty_three, thirty_eight, fifty, sixty_one, seventy_eight, hundred] =
            &mut chunked_outputs;
        chunked.extend_slice_into(
            chunk,
            zero,
            twenty_three,
            thirty_eight,
            fifty,
            sixty_one,
            seventy_eight,
            hundred,
        );
    }
    for level in 0..7 {
        assert_eq!(
            chunked_outputs[level]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            outputs[level]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(chunked.value(), scalar.value());

    bulk.reset();
    assert_eq!(bulk.value(), None);
    let replay = values(bulk.append(input[0]));
    assert_eq!(replay.map(f64::to_bits), expected[0].map(f64::to_bits));
}

#[test]
fn configuration_and_named_levels_are_correct() {
    assert!(FibonacciRetracement::new(0).is_err());
    let mut state = FibonacciRetracement::new(3).unwrap();
    state.append(100.0);
    state.append(80.0);
    let value = state.append(120.0);
    assert_eq!(value.level_zero, 120.0);
    assert_eq!(value.level_twenty_three_point_six, 110.56);
    assert_eq!(value.level_thirty_eight_point_two, 104.72);
    assert_eq!(value.level_fifty, 100.0);
    assert_eq!(value.level_sixty_one_point_eight, 95.28);
    assert_eq!(value.level_seventy_eight_point_six, 88.56);
    assert_eq!(value.level_one_hundred, 80.0);
}
