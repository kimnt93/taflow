use super::average_daily_dollar_value::AverageDailyDollarValue;

#[test]
fn reset_replays_identically() {
    let close: Vec<f64> = (0..100).map(|index| 100.0 + index as f64).collect();
    let volume: Vec<f64> = (0..100).map(|index| 10.0 + index as f64).collect();
    let mut state = AverageDailyDollarValue::new(10).unwrap();
    let first: Vec<u64> = close
        .iter()
        .zip(&volume)
        .map(|(&c, &v)| state.append(c, v).unwrap_or(f64::NAN).to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = close
        .iter()
        .zip(&volume)
        .map(|(&c, &v)| state.append(c, v).unwrap_or(f64::NAN).to_bits())
        .collect();
    assert_eq!(first, second);
}
