use super::gross_profit::GrossProfit;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn sums_strictly_positive_raw_period_pnl() {
    let values = [100.0, -40.0, 0.0, 20.0, -10.0, 5.0];
    let mut state = GrossProfit::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]).unwrap(), Some(100.0));
    state.extend(&values[1..3]).unwrap();
    assert_eq!(state.extend(&values[3..]).unwrap(), Some(125.0));
    assert_eq!(state.compute(), Some(125.0));
    assert_eq!(state.len(), values.len());
}

#[test]
fn closed_trade_pnl_is_consumed_without_conversion() {
    let values = [2500.0, -8000.0, 0.0, 750.0, -20.0];
    let mut state = GrossProfit::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    assert_eq!(state.extend(&values).unwrap(), Some(3250.0));
}

#[test]
fn losses_and_breakevens_produce_valid_zero_after_warmup() {
    let mut state = GrossProfit::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    state.extend(&[-100.0, 0.0, -25.0]).unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state.value(), Some(0.0));
}

#[test]
fn chunking_missing_values_and_reset_are_invariant() {
    let values = [100.0, f64::NAN, -40.0, 0.0, 20.0, -10.0];
    let mut batch = GrossProfit::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);

    let mut streamed = GrossProfit::new(MetricInputKind::RawPnl, NanPolicy::Omit).unwrap();
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
fn rejects_return_domains_and_invalid_observations() {
    assert!(GrossProfit::new(MetricInputKind::Returns, NanPolicy::Omit).is_err());
    assert!(GrossProfit::new(MetricInputKind::LogReturns, NanPolicy::Omit).is_err());
    assert!(GrossProfit::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    assert!(GrossProfit::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .is_err());

    let mut state = GrossProfit::new(MetricInputKind::RawPnl, NanPolicy::Raise).unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.is_empty());
}
