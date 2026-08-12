use super::pain_index::PainIndex;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

// PerformanceAnalytics 2.1.0 source tarball SHA-256:
// fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d.
fn source_convention(returns: &[f64]) -> f64 {
    let mut wealth = 1.0_f64;
    let mut peak = 1.0_f64;
    let mut absolute_drawdown_sum = 0.0;
    for &simple_return in returns {
        wealth *= 1.0 + simple_return;
        peak = peak.max(wealth);
        absolute_drawdown_sum += (wealth / peak - 1.0).abs();
    }
    absolute_drawdown_sum / returns.len() as f64
}

#[test]
fn matches_pinned_performanceanalytics_source_convention() {
    let returns = [0.02, -0.01, 0.015, -0.03, 0.01];
    let mut state = PainIndex::new(NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();
    state.extend(&returns).unwrap();
    assert_relative_eq!(
        state.compute().unwrap(),
        source_convention(&returns),
        epsilon = 1e-15
    );
}

#[test]
fn phantom_wealth_and_real_observation_divisor_are_explicit() {
    let mut loss = PainIndex::new(NanPolicy::Omit).unwrap();
    loss.from_returns(&[]).unwrap();
    loss.append(-0.2).unwrap();
    assert_relative_eq!(loss.value().unwrap(), 0.2, epsilon = 1e-15);

    let mut gain = PainIndex::new(NanPolicy::Omit).unwrap();
    gain.from_returns(&[]).unwrap();
    gain.append(0.2).unwrap();
    assert_eq!(gain.value(), Some(0.0));
}

#[test]
fn lifecycle_omission_and_reset_are_invariant() {
    let returns = [0.10, f64::NAN, -0.20, 0.05, -0.25, 0.10];
    let mut batch = PainIndex::new(NanPolicy::Omit).unwrap();
    batch.from_returns(&[]).unwrap();
    batch.extend(&returns).unwrap();
    assert_eq!(batch.len(), 5);
    let expected = batch.value().unwrap();

    let mut streamed = PainIndex::new(NanPolicy::Omit).unwrap();
    streamed.from_returns(&[]).unwrap();
    for value in returns {
        streamed.append(value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&returns).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn rejects_non_path_domains_and_invalid_observations() {
    let mut unbound = PainIndex::new(NanPolicy::Omit).unwrap();
    assert!(unbound.append(0.01).is_err());

    let mut state = PainIndex::new(NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.is_empty());
}
