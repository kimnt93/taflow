use super::win_rate::WinRate;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn matches_quantstats_decisive_observation_definition() {
    // QuantStats 0.0.81 win_rate with preparation disabled divides strictly
    // positive observations by non-zero observations. Zeros are breakevens and
    // remain part of len(), but are excluded from both numerator and denominator.
    let mut metric = WinRate::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    metric.extend(&[0.10, 0.0, -0.20, 0.30, 0.0]).unwrap();

    assert_eq!(metric.value(), Some(2.0 / 3.0));
    assert_eq!(metric.compute(), metric.value());
    assert_eq!(metric.len(), 5);
}

#[test]
fn return_period_pnl_and_trade_domains_preserve_observation_meaning() {
    let values = [10.0, 0.0, -20.0, 30.0, -5.0];

    let mut returns = WinRate::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    returns.extend(&[0.10, 0.0, -0.20, 0.30, -0.05]).unwrap();

    let mut pnl = WinRate::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    pnl.extend(&values).unwrap();

    let mut trades = WinRate::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    trades.extend(&values).unwrap();

    assert_eq!(returns.value(), Some(0.5));
    assert_eq!(pnl.value(), returns.value());
    assert_eq!(trades.value(), returns.value());
}

#[test]
fn lifecycle_missing_values_and_breakevens_are_invariant() {
    let values = [0.10, f64::NAN, 0.0, -0.20, 0.30, 0.0];
    let mut batch = WinRate::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);
    assert_eq!(batch.value(), Some(2.0 / 3.0));

    let mut streamed = WinRate::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    streamed.append(values[0]).unwrap();
    streamed.extend(&values[1..4]).unwrap();
    streamed.extend(&values[4..]).unwrap();
    assert_eq!(streamed.compute(), batch.compute());

    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_eq!(streamed.compute(), batch.compute());
}

#[test]
fn empty_and_only_breakeven_samples_are_undefined() {
    let mut metric = WinRate::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(metric.value(), None);
    metric.extend(&[0.0, -0.0, 0.0]).unwrap();
    assert_eq!(metric.len(), 3);
    assert_eq!(metric.value(), None);
}

#[test]
fn rejects_ineligible_domains_and_invalid_observations() {
    assert!(WinRate::new(MetricInputKind::LogReturns, NanPolicy::Omit).is_err());
    assert!(WinRate::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    assert!(WinRate::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .is_err());

    let mut state = WinRate::new(MetricInputKind::Returns, NanPolicy::Raise).unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}
