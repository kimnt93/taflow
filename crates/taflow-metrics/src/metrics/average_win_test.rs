use super::average_win::AverageWin;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_mean_of_strictly_positive_returns_and_preserves_lifecycle() {
    let values = [0.02, -0.01, 0.0, 0.03, -0.025, 0.01];
    let expected = (0.02 + 0.03 + 0.01) / 3.0;
    let mut state = AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]).unwrap(), Some(values[0]));
    state.extend(&values[1..3]).unwrap();
    state.append(values[3]).unwrap();
    assert_close(state.extend(&values[4..]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), values.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&values).unwrap().unwrap(), expected);
}

#[test]
fn raw_period_pnl_and_closed_trades_are_not_converted() {
    let observations = [100.0, -40.0, 0.0, 20.0, -10.0];
    let expected = 60.0;

    let mut pnl = AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(pnl.extend(&observations).unwrap().unwrap(), expected);

    let mut trades = AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(trades.extend(&observations).unwrap().unwrap(), expected);
}

#[test]
fn losses_and_breakevens_do_not_create_a_result() {
    let mut state = AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[-0.10, 0.0, -0.02]).unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state.value(), None);

    assert_eq!(state.append(0.05).unwrap(), Some(0.05));
}

#[test]
fn missing_and_invalid_values_follow_the_input_contract() {
    let mut omit = AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    omit.extend(&[f64::NAN, 0.02, -0.01]).unwrap();
    assert_eq!(omit.len(), 2);
    assert_eq!(omit.value(), Some(0.02));

    let mut raise = AverageWin::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    raise.append(0.01).unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert_eq!(raise.len(), 1);
    assert_eq!(raise.value(), Some(0.01));

    assert!(omit.append(f64::INFINITY).is_err());
    assert!(omit.append(-1.01).is_err());
    assert_eq!(omit.len(), 2);
}

#[test]
fn rejects_return_converters_outside_the_declared_domains() {
    assert!(AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(AverageWin::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
