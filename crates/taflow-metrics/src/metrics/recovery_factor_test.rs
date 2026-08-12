use super::recovery_factor::RecoveryFactor;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-12 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn freezes_quantstats_arithmetic_sum_definition() {
    // QuantStats 0.0.81 recovery_factor with prepare_returns=False computes
    // abs(sum(returns) - rf) / abs(max_drawdown). TAFlow freezes rf at zero.
    // The arithmetic sum (0.10) deliberately differs from compounded total
    // return (0.089) for this path, so this test detects a definition change.
    let returns = [0.10, -0.10, 0.10];
    let mut metric = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    metric.extend(&returns).unwrap();

    assert_close(metric.value().unwrap(), 1.0);
    assert_close(metric.compute().unwrap(), 1.0);
    assert_eq!(metric.len(), returns.len());
}

#[test]
fn semantic_input_modes_and_streaming_are_invariant() {
    let returns: [f64; 5] = [0.10, -0.20, 0.05, -0.25, 0.10];
    let equity = [100.0, 110.0, 88.0, 92.4, 69.3, 76.23];
    let pnl = [10.0, -22.0, 4.4, -23.1, 6.93];
    let log_returns: Vec<f64> = returns.iter().map(|value| value.ln_1p()).collect();

    let mut expected = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    expected.extend(&returns).unwrap();
    let expected_value = expected.value().unwrap();

    let mut from_equity = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    from_equity.extend(&equity).unwrap();
    assert_close(from_equity.value().unwrap(), expected_value);

    let mut from_pnl = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.0)?;
            Ok(state)
        })
        .unwrap();
    from_pnl.extend(&pnl).unwrap();
    assert_close(from_pnl.value().unwrap(), expected_value);

    let mut from_log = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    from_log.extend(&log_returns).unwrap();
    assert_close(from_log.value().unwrap(), expected_value);

    let mut streamed = RecoveryFactor::new(NanPolicy::Omit)
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
fn undefined_and_missing_value_contract_is_explicit() {
    let mut empty = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(empty.value(), None);
    empty.extend(&[0.10, 0.0, 0.20]).unwrap();
    assert_eq!(empty.value(), None);

    let mut omitted = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    omitted.extend(&[f64::NAN, -0.10, f64::NAN]).unwrap();
    assert_eq!(omitted.len(), 1);
    assert_close(omitted.value().unwrap(), 1.0);

    let mut zero_numerator = RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    zero_numerator.extend(&[-0.10, 0.10]).unwrap();
    assert_eq!(zero_numerator.value(), Some(0.0));
}

#[test]
fn rejects_ineligible_domains_and_invalid_observations() {
    assert!(RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(RecoveryFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = RecoveryFactor::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}
