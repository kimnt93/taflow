use super::average_loss::AverageLoss;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_signed_mean_of_strictly_negative_returns() {
    let values = [0.02, -0.01, 0.0, 0.03, -0.025, -0.01];
    let expected = (-0.01 - 0.025 - 0.01) / 3.0;
    let mut state = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]).unwrap(), None);
    state.extend(&values[1..3]).unwrap();
    assert_eq!(state.value(), Some(-0.01));
    assert_close(state.extend(&values[3..]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert!(state.compute().unwrap() < 0.0);
    assert_eq!(state.len(), values.len());
}

#[test]
fn raw_period_pnl_and_closed_trades_are_not_converted() {
    let observations = [100.0, -40.0, 0.0, 20.0, -10.0];
    let expected = -25.0;

    let mut pnl = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(pnl.extend(&observations).unwrap().unwrap(), expected);

    let mut trades = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(trades.extend(&observations).unwrap().unwrap(), expected);
}

#[test]
fn wins_and_breakevens_do_not_create_a_result() {
    let mut state = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[0.10, 0.0, 0.02]).unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state.value(), None);

    assert_eq!(state.append(-0.05).unwrap(), Some(-0.05));
}

#[test]
fn chunking_missing_values_and_reset_are_invariant() {
    let values = [0.02, f64::NAN, -0.01, 0.0, -0.025, -0.01];
    let mut batch = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&values).unwrap();
    let expected = batch.value().unwrap();
    assert_eq!(batch.len(), 5);

    let mut streamed = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    streamed.append(values[0]).unwrap();
    streamed.extend(&values[1..4]).unwrap();
    streamed.extend(&values[4..]).unwrap();
    assert_close(streamed.compute().unwrap(), expected);

    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_close(streamed.compute().unwrap(), expected);
}

#[test]
fn rejects_ineligible_domains_and_invalid_observations() {
    assert!(AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = AverageLoss::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());

    let mut raw = AverageLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(raw.append(-1000.0).unwrap(), Some(-1000.0));
}
