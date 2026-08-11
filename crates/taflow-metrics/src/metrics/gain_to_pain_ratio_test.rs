use super::gain_to_pain_ratio::GainToPainRatio;
use crate::{MetricInputKind, NanPolicy};

fn expected_quantstats_definition(returns: &[f64]) -> f64 {
    // QuantStats 0.0.81 gain_to_pain_ratio sums all returns in the numerator,
    // not only positive returns. Phase 1 receives observations already at the
    // caller's intended aggregation resolution instead of resampling dates.
    let total: f64 = returns.iter().sum();
    let pain: f64 = -returns.iter().filter(|value| **value < 0.0).sum::<f64>();
    total / pain
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_frozen_quantstats_definition_and_preserves_lifecycle() {
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01];
    let expected = expected_quantstats_definition(&returns);
    let mut state = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    state.append(returns[0]).unwrap();
    assert_eq!(state.value(), None);
    state.extend(&returns[1..3]).unwrap();
    state.append(returns[3]).unwrap();
    assert_close(state.append(returns[4]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), returns.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn all_return_input_modes_are_equivalent() {
    let returns = [0.10, -0.20, 0.05];
    let expected = expected_quantstats_definition(&returns);

    let mut equity = GainToPainRatio::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = GainToPainRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let logarithmic_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        GainToPainRatio::new(MetricInputKind::LogReturns, NanPolicy::Omit).unwrap();
    assert_close(
        logarithmic.extend(&logarithmic_returns).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn handles_missing_values_and_zero_pain() {
    let mut state = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, 0.02, -0.01]).unwrap();
    assert_eq!(state.len(), 2);
    assert_close(state.value().unwrap(), 1.0);

    let mut no_losses = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    no_losses.extend(&[0.01, 0.02, 0.0]).unwrap();
    assert_eq!(no_losses.value(), None);

    let mut only_losses = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    only_losses.extend(&[-0.01, -0.02]).unwrap();
    assert_eq!(only_losses.value(), Some(-1.0));
}

#[test]
fn raise_policy_rejects_nan_without_mutating_state() {
    let mut state = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Raise).unwrap();
    state.append(-0.01).unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), Some(-1.0));
}

#[test]
fn validates_values_and_semantic_domain() {
    assert!(GainToPainRatio::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    assert!(GainToPainRatio::new(MetricInputKind::Trades, NanPolicy::Omit).is_err());

    let mut state = GainToPainRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
}
