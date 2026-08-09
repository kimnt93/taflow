use super::heikin_ashi::{HeikinAshi, HeikinAshiValue};

#[test]
fn heikin_ashi_matches_expected_and_preserves_lifecycle() {
    let open = [10.0, 12.0, 14.0];
    let high = [13.0, 15.0, 16.0];
    let low = [9.0, 11.0, 12.0];
    let close = [12.0, 14.0, 13.0];
    let expected = [
        HeikinAshiValue {
            open: 11.0,
            high: 13.0,
            low: 9.0,
            close: 11.0,
        },
        HeikinAshiValue {
            open: 11.0,
            high: 15.0,
            low: 11.0,
            close: 13.0,
        },
        HeikinAshiValue {
            open: 12.0,
            high: 16.0,
            low: 12.0,
            close: 13.75,
        },
    ];
    let mut state = HeikinAshi::new().unwrap();
    for index in 0..open.len() {
        assert_eq!(
            state.append(open[index], high[index], low[index], close[index]),
            expected[index]
        );
    }
    assert_eq!(state.value(), Some(expected[2]));

    state.reset();
    let (mut output_open, mut output_high, mut output_low, mut output_close) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    state
        .extend_slices_into(
            &open,
            &high,
            &low,
            &close,
            &mut output_open,
            &mut output_high,
            &mut output_low,
            &mut output_close,
        )
        .unwrap();
    assert_eq!(output_open, expected.map(|value| value.open));
    assert_eq!(output_high, expected.map(|value| value.high));
    assert_eq!(output_low, expected.map(|value| value.low));
    assert_eq!(output_close, expected.map(|value| value.close));
}

#[test]
fn heikin_ashi_rejects_misaligned_slices_before_mutation() {
    let mut state = HeikinAshi::new().unwrap();
    let (mut output_open, mut output_high, mut output_low, mut output_close) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    assert!(state
        .extend_slices_into(
            &[1.0],
            &[],
            &[1.0],
            &[1.0],
            &mut output_open,
            &mut output_high,
            &mut output_low,
            &mut output_close,
        )
        .is_err());
    assert_eq!(state.value(), None);
    assert!(output_open.is_empty());
}
