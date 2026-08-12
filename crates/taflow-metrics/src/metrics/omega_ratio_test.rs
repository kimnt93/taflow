use super::omega_ratio::OmegaRatio;
use crate::{MetricInputKind, NanPolicy};

fn expected_omega(returns: &[f64], periods_per_year: f64, annual_required_return: f64) -> f64 {
    let threshold = (annual_required_return.ln_1p() / periods_per_year).exp_m1();
    let mut gains = 0.0;
    let mut losses = 0.0;
    for value in returns {
        let excess = value - threshold;
        if excess > 0.0 {
            gains += excess;
        } else if excess < 0.0 {
            losses -= excess;
        }
    }
    gains / losses
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_empyrical_definition_and_preserves_lifecycle() {
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01];
    let expected = expected_omega(&returns, 12.0, 0.06);
    let mut state = OmegaRatio::new(12.0, 0.06, NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(returns[0]).unwrap(), None);
    state.extend(&returns[1..3]).unwrap();
    state.append(returns[3]).unwrap();
    assert_close(state.append(returns[4]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), returns.len());
    assert_eq!(state.periods_per_year(), 12.0);
    assert_eq!(state.annual_required_return(), 0.06);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn all_return_input_modes_are_equivalent() {
    let returns = [0.10, -0.20, 0.05];
    let expected = expected_omega(&returns, 12.0, 0.03);

    let mut equity = OmegaRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    equity.from_equity(&[]).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = OmegaRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    pnl.from_pnl(&[], 100.0).unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let logarithmic_returns = returns.map(f64::ln_1p);
    let mut logarithmic = OmegaRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    logarithmic.from_log_returns(&[]).unwrap();
    assert_close(
        logarithmic.extend(&logarithmic_returns).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn handles_missing_minimum_sample_and_zero_denominator() {
    let mut state = OmegaRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    state.from_returns(&[]).unwrap();
    state.extend(&[f64::NAN, -0.01]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), None);
    state.append(0.02).unwrap();
    assert!(state.value().is_some());

    let mut no_losses = OmegaRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    no_losses.from_returns(&[]).unwrap();
    no_losses.extend(&[0.01, 0.02]).unwrap();
    assert_eq!(no_losses.value(), None);

    let mut no_gains = OmegaRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    no_gains.from_returns(&[]).unwrap();
    no_gains.extend(&[-0.01, -0.02]).unwrap();
    assert_eq!(no_gains.value(), Some(0.0));
}

#[test]
fn validates_configuration_and_semantic_domain() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(OmegaRatio::new(invalid, 0.0, NanPolicy::Omit).is_err());
    }
    for invalid in [-1.0, -2.0, f64::NAN, f64::INFINITY] {
        assert!(OmegaRatio::new(252.0, invalid, NanPolicy::Omit).is_err());
    }
    let mut unbound = OmegaRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    assert!(unbound.append(0.01).is_err());
}
