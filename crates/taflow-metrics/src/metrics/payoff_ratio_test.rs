use super::payoff_ratio::PayoffRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_average_win_over_absolute_average_loss_and_preserves_lifecycle() {
    let values = [0.02, -0.01, 0.0, 0.03, -0.025, 0.01];
    let average_win = (0.02 + 0.03 + 0.01) / 3.0;
    let average_loss = (-0.01 - 0.025) / 2.0;
    let expected = average_win / -average_loss;
    let mut state = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]).unwrap(), None);
    assert!(state.append(values[1]).unwrap().is_some());
    state.extend(&values[2..4]).unwrap();
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
    let expected = 60.0 / 25.0;

    let mut pnl = PayoffRatio::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    assert_close(pnl.extend(&observations).unwrap().unwrap(), expected);

    let mut trades = PayoffRatio::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    assert_close(trades.extend(&observations).unwrap().unwrap(), expected);
}

#[test]
fn either_missing_side_keeps_the_ratio_undefined() {
    let mut wins = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    wins.extend(&[0.10, 0.0, 0.02]).unwrap();
    assert_eq!(wins.value(), None);

    let mut losses = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    losses.extend(&[-0.10, 0.0, -0.02]).unwrap();
    assert_eq!(losses.value(), None);

    let mut breakeven = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    breakeven.extend(&[0.0, 0.0]).unwrap();
    assert_eq!(breakeven.value(), None);
}

#[test]
fn missing_and_invalid_values_follow_the_input_contract() {
    let mut omit = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    omit.extend(&[f64::NAN, 0.02, -0.01]).unwrap();
    assert_eq!(omit.len(), 2);
    assert_eq!(omit.value(), Some(2.0));

    let mut raise = PayoffRatio::new(MetricInputKind::Returns, NanPolicy::Raise).unwrap();
    raise.extend(&[0.01, -0.02]).unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert_eq!(raise.len(), 2);
    assert_eq!(raise.value(), Some(0.5));

    assert!(omit.append(f64::INFINITY).is_err());
    assert!(omit.append(-1.01).is_err());
    assert_eq!(omit.len(), 2);
}

#[test]
fn rejects_return_converters_outside_the_declared_domains() {
    assert!(PayoffRatio::new(MetricInputKind::LogReturns, NanPolicy::Omit).is_err());
    assert!(PayoffRatio::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    assert!(PayoffRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .is_err());
}
