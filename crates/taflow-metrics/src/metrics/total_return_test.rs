use super::total_return::TotalReturn;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn compounds_returns_and_preserves_lifecycle() {
    let values = [0.10, -0.20, 0.05];
    let expected = 1.10 * 0.80 * 1.05 - 1.0;
    let mut state = TotalReturn::new(NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();

    assert_eq!(state.value(), None);
    state.extend(&values[..2]).unwrap();
    assert_close(state.append(values[2]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), 3);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&values).unwrap().unwrap(), expected);
}

#[test]
fn input_modes_produce_equivalent_returns() {
    let returns = [0.10, -0.20, 0.05];
    let expected = 1.10 * 0.80 * 1.05 - 1.0;

    let mut equity = TotalReturn::new(NanPolicy::Omit).unwrap();
    equity.from_equity(&[]).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = TotalReturn::new(NanPolicy::Omit).unwrap();
    pnl.from_pnl(&[], 100.0).unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic = TotalReturn::new(NanPolicy::Omit).unwrap();
    logarithmic.from_log_returns(&[]).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn omits_nan_and_handles_total_loss() {
    let mut state = TotalReturn::new(NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();
    state.extend(&[f64::NAN, 0.25, -1.0]).unwrap();
    assert_eq!(state.value(), Some(-1.0));
    assert_eq!(state.len(), 2);
}

#[test]
fn rejects_non_return_semantic_domains() {
    let mut unbound = TotalReturn::new(NanPolicy::Omit).unwrap();
    assert!(unbound.append(0.01).is_err());
}
