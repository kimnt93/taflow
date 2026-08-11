use super::expectancy::Expectancy;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

#[test]
fn computes_component_expectancy_over_all_observations() {
    let mut state = Expectancy::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    state.extend(&[100.0, -40.0, 0.0, 20.0, -10.0]).unwrap();
    // (2/5 * 60) + (2/5 * -25) + (1/5 * 0) = 14.
    assert_relative_eq!(state.compute().unwrap(), 14.0, epsilon = 1e-15);
}

#[test]
fn breakevens_are_in_probability_denominator() {
    let mut without_breakeven = Expectancy::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    without_breakeven.extend(&[100.0, -40.0]).unwrap();
    assert_relative_eq!(without_breakeven.value().unwrap(), 30.0, epsilon = 1e-15);

    let mut with_breakeven = Expectancy::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    with_breakeven.extend(&[100.0, -40.0, 0.0]).unwrap();
    assert_relative_eq!(with_breakeven.value().unwrap(), 20.0, epsilon = 1e-15);
}

#[test]
fn raw_period_and_trade_domains_have_identical_arithmetic() {
    let values = [250.0, -125.0, 0.0, 75.0, -50.0];
    let mut period = Expectancy::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    let mut trades = Expectancy::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    period.extend(&values).unwrap();
    trades.extend(&values).unwrap();
    assert_eq!(period.compute(), trades.compute());
}

#[test]
fn lifecycle_omission_and_reset_are_invariant() {
    let values = [100.0, f64::NAN, -40.0, 0.0, 20.0, -10.0];
    let mut batch = Expectancy::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);
    let expected = batch.value().unwrap();

    let mut streamed = Expectancy::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    for value in values {
        streamed.append(value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn freezes_empty_and_all_breakeven_results_and_rejects_other_domains() {
    let mut state = Expectancy::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    assert_eq!(state.value(), None);
    state.extend(&[0.0, -0.0, 0.0]).unwrap();
    assert_eq!(state.value(), Some(0.0));

    assert!(Expectancy::new(MetricInputKind::Returns, NanPolicy::Omit).is_err());
    assert!(Expectancy::new(MetricInputKind::LogReturns, NanPolicy::Omit).is_err());
    assert!(Expectancy::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    assert!(Expectancy::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .is_err());
}
