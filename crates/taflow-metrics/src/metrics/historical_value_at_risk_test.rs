use super::historical_value_at_risk::HistoricalValueAtRisk;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn computes_linear_quantile_and_refreshes_dirty_cache() {
    let mut state =
        HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.25, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(-0.04).unwrap(), Some(-0.04));
    state.extend(&[0.03, -0.02, 0.01]).unwrap();
    assert_close(state.value().unwrap(), -0.025);
    assert_close(state.compute().unwrap(), -0.025);

    state.append(-0.10).unwrap();
    assert_close(state.compute().unwrap(), -0.04);
    assert_eq!(state.len(), 5);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.compute(), None);
    state.extend(&[-0.04, 0.03, -0.02, 0.01]).unwrap();
    assert_close(state.value().unwrap(), -0.025);
}

#[test]
fn scalar_and_batch_updates_are_invariant() {
    let values = [-0.07, 0.02, -0.01, 0.04, -0.03, 0.0];
    let mut scalar =
        HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.3, NanPolicy::Omit).unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }

    let mut batch =
        HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.3, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();

    assert_eq!(scalar.len(), batch.len());
    assert_eq!(scalar.compute(), batch.compute());
}

#[test]
fn input_modes_produce_equivalent_quantiles() {
    let returns = [0.10, -0.20, 0.05];
    let expected = {
        let mut state =
            HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.4, NanPolicy::Omit).unwrap();
        state.extend(&returns).unwrap().unwrap()
    };

    let mut equity =
        HistoricalValueAtRisk::new(MetricInputKind::Equity, 0.4, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = HistoricalValueAtRisk::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        0.4,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        HistoricalValueAtRisk::new(MetricInputKind::LogReturns, 0.4, NanPolicy::Omit).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn handles_missing_values_and_one_observation_minimum() {
    let mut state =
        HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, -0.07, f64::NAN]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), Some(-0.07));

    let mut raising =
        HistoricalValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Raise).unwrap();
    assert!(raising.append(f64::NAN).is_err());
    assert_eq!(raising.len(), 0);
}

#[test]
fn rejects_invalid_cutoffs_and_non_return_domains() {
    for cutoff in [0.0, 1.0, -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(
            HistoricalValueAtRisk::new(MetricInputKind::Returns, cutoff, NanPolicy::Omit).is_err()
        );
    }
    assert!(HistoricalValueAtRisk::new(MetricInputKind::RawPnl, 0.05, NanPolicy::Omit).is_err());
    assert!(HistoricalValueAtRisk::new(MetricInputKind::Trades, 0.05, NanPolicy::Omit).is_err());
}
