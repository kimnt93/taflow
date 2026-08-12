use super::sortino_ratio::SortinoRatio;
use crate::NanPolicy;

fn expected_sortino(returns: &[f64], periods_per_year: f64, annual_required_return: f64) -> f64 {
    let required = (annual_required_return.ln_1p() / periods_per_year).exp_m1();
    let excess_sum: f64 = returns.iter().map(|value| value - required).sum();
    let squared_shortfall_sum: f64 = returns
        .iter()
        .map(|value| (value - required).min(0.0).powi(2))
        .sum();
    let annualized_excess = excess_sum / returns.len() as f64 * periods_per_year;
    let annualized_downside =
        (squared_shortfall_sum / returns.len() as f64).sqrt() * periods_per_year.sqrt();
    annualized_excess / annualized_downside
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_empyrical_definition_and_preserves_lifecycle() {
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01];
    let expected = expected_sortino(&returns, 12.0, 0.06);
    let mut state = SortinoRatio::new(12.0, 0.06, NanPolicy::Omit).unwrap();
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
    let expected = expected_sortino(&returns, 12.0, 0.03);

    let mut equity = SortinoRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    equity.from_equity(&[100.0, 110.0, 88.0, 92.4]).unwrap();
    assert_close(equity.compute().unwrap(), expected);
    assert_eq!(equity.len(), 3);

    let mut pnl = SortinoRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    pnl.from_pnl(&[10.0, -22.0, 4.4], 100.0).unwrap();
    assert_close(pnl.compute().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic = SortinoRatio::new(12.0, 0.03, NanPolicy::Omit).unwrap();
    logarithmic.from_log_returns(&log_returns).unwrap();
    assert_close(logarithmic.compute().unwrap(), expected);
}

#[test]
fn handles_missing_minimum_sample_and_zero_downside() {
    let mut state = SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    state.from_returns(&[f64::NAN, -0.01]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), None);
    state.append(0.02).unwrap();
    assert!(state.value().is_some());

    let mut no_downside = SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    no_downside.from_returns(&[0.01, 0.02]).unwrap();
    assert_eq!(no_downside.value(), None);
}

#[test]
fn validates_configuration_and_semantic_domain() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(SortinoRatio::new(invalid, 0.0, NanPolicy::Omit).is_err());
    }
    for invalid in [-1.0, -2.0, f64::NAN, f64::INFINITY] {
        assert!(SortinoRatio::new(252.0, invalid, NanPolicy::Omit).is_err());
    }
    let mut unbound = SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap();
    assert!(unbound.append(0.01).is_err());
}
