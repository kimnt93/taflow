use super::gross_loss::GrossLoss;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn sums_strictly_negative_period_pnl_and_preserves_lifecycle() {
    let values = [100.0, -40.0, 0.0, 20.0, -10.0];
    let mut state = GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]).unwrap(), Some(0.0));
    state.extend(&values[1..3]).unwrap();
    assert_eq!(state.append(values[3]).unwrap(), Some(-40.0));
    assert_eq!(state.extend(&values[4..]).unwrap(), Some(-50.0));
    assert_eq!(state.compute(), Some(-50.0));
    assert_eq!(state.len(), values.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_eq!(state.extend(&values).unwrap(), Some(-50.0));
}

#[test]
fn closed_trade_pnl_uses_the_same_unconverted_signed_sum() {
    let values = [250.0, -125.0, -75.0, 50.0];
    let mut state = GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.extend(&values).unwrap(), Some(-200.0));
    assert_eq!(state.len(), values.len());
}

#[test]
fn nonempty_histories_without_losses_return_zero() {
    let mut state = GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[0.0, 20.0, 100.0]).unwrap();
    assert_eq!(state.value(), Some(0.0));
}

#[test]
fn missing_and_invalid_values_follow_the_input_contract() {
    let mut omit = GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    omit.extend(&[f64::NAN, -1000.0, 50.0]).unwrap();
    assert_eq!(omit.len(), 2);
    assert_eq!(omit.value(), Some(-1000.0));

    let mut raise = GrossLoss::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    raise.append(-10.0).unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert_eq!(raise.len(), 1);
    assert_eq!(raise.value(), Some(-10.0));

    assert!(omit.append(f64::INFINITY).is_err());
    assert_eq!(omit.len(), 2);
}

#[test]
fn rejects_returns_and_return_conversion_domains() {
    assert!(GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(GrossLoss::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
