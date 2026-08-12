use super::ulcer_performance_index::UlcerPerformanceIndex;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-12 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn freezes_quantstats_compounded_unannualized_definition() {
    // QuantStats 0.0.81 ulcer_performance_index computes
    // (prod(1 + returns) - 1 - rf) / ulcer_index. TAFlow freezes rf at
    // zero, exposes no period or annualization convention, and retains the
    // oracle's n - 1 ulcer-index divisor. The compounded numerator here is
    // 0.089, deliberately different from the arithmetic sum of 0.10.
    let returns = [0.10, -0.10, 0.10];
    let mut metric = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    metric.extend(&returns).unwrap();

    let ulcer_index = (0.0101_f64 / 2.0).sqrt();
    assert_close(metric.value().unwrap(), 0.089 / ulcer_index);
    assert_eq!(metric.compute(), metric.value());
    assert_eq!(metric.len(), returns.len());
}

#[test]
fn semantic_input_modes_and_streaming_are_invariant() {
    let returns: [f64; 5] = [0.10, -0.20, 0.05, -0.25, 0.10];
    let equity = [100.0, 110.0, 88.0, 92.4, 69.3, 76.23];
    let pnl = [10.0, -22.0, 4.4, -23.1, 6.93];
    let log_returns: Vec<f64> = returns.iter().map(|value| value.ln_1p()).collect();

    let mut expected = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    expected.extend(&returns).unwrap();
    let expected_value = expected.value().unwrap();

    let mut from_equity = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    from_equity.extend(&equity).unwrap();
    assert_close(from_equity.value().unwrap(), expected_value);

    let mut from_pnl = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.0)?;
            Ok(state)
        })
        .unwrap();
    from_pnl.extend(&pnl).unwrap();
    assert_close(from_pnl.value().unwrap(), expected_value);

    let mut from_log = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    from_log.extend(&log_returns).unwrap();
    assert_close(from_log.value().unwrap(), expected_value);

    let mut streamed = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    streamed.append(returns[0]).unwrap();
    streamed.extend(&returns[1..3]).unwrap();
    streamed.extend(&returns[3..]).unwrap();
    assert_close(streamed.value().unwrap(), expected_value);

    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&returns).unwrap();
    assert_close(streamed.value().unwrap(), expected_value);
}

#[test]
fn warmup_zero_risk_and_missing_contract_is_explicit() {
    let mut state = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.value(), None);
    state.append(-0.10).unwrap();
    assert_eq!(state.value(), None);

    let mut zero_risk = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    zero_risk.extend(&[0.10, 0.0, 0.20]).unwrap();
    assert_eq!(zero_risk.value(), None);

    let mut omitted = UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    omitted.extend(&[f64::NAN, -0.10, -0.10, f64::NAN]).unwrap();
    assert_eq!(omitted.len(), 2);
    assert!(omitted.value().is_some());
}

#[test]
fn rejects_ineligible_domains_and_invalid_observations() {
    assert!(UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(UlcerPerformanceIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = UlcerPerformanceIndex::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}
