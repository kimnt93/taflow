use super::tail_ratio::TailRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-13, "{actual} != {expected}");
}

#[test]
fn computes_linear_tail_quantiles_and_refreshes_dirty_cache() {
    let values = [-0.10, -0.04, -0.01, 0.0, 0.02, 0.06, 0.15];
    let mut state = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    state.extend(&values).unwrap();
    let lower: f64 = -0.10 + (-0.04 - -0.10) * 0.3;
    let upper: f64 = 0.06 + (0.15 - 0.06) * 0.7;
    assert_close(state.compute().unwrap(), upper.abs() / lower.abs());

    state.append(-0.20).unwrap();
    assert!(state.value().unwrap().is_finite());
    assert_eq!(state.len(), values.len() + 1);
}

#[test]
fn scalar_chunked_and_batch_updates_are_invariant() {
    let values = [-0.07, 0.02, -0.01, 0.04, -0.03, 0.0, 0.08];
    let mut scalar = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }

    let mut chunked = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    chunked.extend(&values[..3]).unwrap();
    chunked.extend(&values[3..]).unwrap();

    let mut batch = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(scalar.len(), batch.len());
    assert_eq!(scalar.compute(), chunked.compute());
    assert_eq!(scalar.compute(), batch.compute());
}

#[test]
fn input_modes_produce_equivalent_ratios() {
    let returns = [0.10, -0.20, 0.05];
    let expected = {
        let mut state = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
        state.extend(&returns).unwrap().unwrap()
    };

    let mut equity = TailRatio::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = TailRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic = TailRatio::new(MetricInputKind::LogReturns, NanPolicy::Omit).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn handles_warmup_missing_values_zero_denominator_and_reset() {
    let mut state = TailRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, -0.07, f64::NAN]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), Some(1.0));

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.compute(), None);
    state.extend(&[0.0, 0.0]).unwrap();
    assert_eq!(state.compute(), None);

    let mut raising = TailRatio::new(MetricInputKind::Returns, NanPolicy::Raise).unwrap();
    assert!(raising.append(f64::NAN).is_err());
    assert_eq!(raising.len(), 0);
}

#[test]
fn rejects_non_return_domains() {
    assert!(TailRatio::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    assert!(TailRatio::new(MetricInputKind::Trades, NanPolicy::Omit).is_err());
}
