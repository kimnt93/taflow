use super::modified_sharpe_ratio::ModifiedSharpeRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn computes_frozen_cornish_fisher_modified_var_ratio() {
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.03];
    // Translation of PerformanceAnalytics 2.1.0 SharpeRatio(FUN="VaR",
    // method="modified", annualize=FALSE, geometric=FALSE, invert=FALSE).
    let expected = 0.049_235_041_153_778_45;
    let mut state = ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.value(), None);
    state.extend(&returns[..3]).unwrap();
    state.extend(&returns[3..]).unwrap();
    assert_close(state.compute().unwrap(), expected, 2e-10);
    assert_eq!(state.len(), returns.len());
}

#[test]
fn all_return_input_modes_and_lifecycle_are_equivalent() {
    let returns = [0.10, -0.20, 0.05, -0.03];
    let settings = (12.0, 0.03, 0.975);
    let mut direct = ModifiedSharpeRatio::new(settings.0, settings.1, settings.2, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let expected = direct.extend(&returns).unwrap().unwrap();

    let mut equity = ModifiedSharpeRatio::new(settings.0, settings.1, settings.2, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4, 89.628])
            .unwrap()
            .unwrap(),
        expected,
        1e-10,
    );

    let mut pnl = ModifiedSharpeRatio::new(settings.0, settings.1, settings.2, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.0)?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4, -2.772]).unwrap().unwrap(),
        expected,
        1e-10,
    );

    let mut logarithmic =
        ModifiedSharpeRatio::new(settings.0, settings.1, settings.2, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_log_returns(&[])?;
                Ok(state)
            })
            .unwrap();
    let logarithmic_returns = returns.map(f64::ln_1p);
    assert_close(
        logarithmic.extend(&logarithmic_returns).unwrap().unwrap(),
        expected,
        1e-10,
    );

    direct.reset();
    assert!(direct.is_empty());
    assert_eq!(direct.value(), None);
    assert_close(direct.extend(&returns).unwrap().unwrap(), expected, 1e-12);
}

#[test]
fn handles_minimum_constant_and_inverse_risk_boundaries() {
    let mut state = ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.append(-0.01).unwrap(), None);
    assert_eq!(state.append(-0.01).unwrap(), Some(-1.0));

    state.reset();
    state.extend(&[0.01, 0.01]).unwrap();
    assert_eq!(state.value(), None);

    state.reset();
    state.extend(&[0.0, 0.0]).unwrap();
    assert_eq!(state.value(), None);
}

#[test]
fn missing_and_invalid_values_follow_the_input_contract() {
    let mut omit = ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    omit.extend(&[f64::NAN, 0.02, -0.01]).unwrap();
    assert_eq!(omit.len(), 2);
    assert!(omit.value().is_some());

    let mut raise = ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert!(raise.is_empty());
    assert!(omit.append(f64::INFINITY).is_err());
    assert!(omit.append(-1.01).is_err());
}

#[test]
fn validates_configuration_and_semantic_domain() {
    for periods in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            ModifiedSharpeRatio::new(periods, 0.0, 0.95, NanPolicy::Omit)
                .and_then(|mut state| {
                    state.from_returns(&[])?;
                    Ok(state)
                })
                .is_err()
        );
    }
    for rate in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(ModifiedSharpeRatio::new(252.0, rate, 0.95, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .is_err());
    }
    for confidence in [0.5, 1.0, f64::NAN] {
        assert!(
            ModifiedSharpeRatio::new(252.0, 0.0, confidence, NanPolicy::Omit)
                .and_then(|mut state| {
                    state.from_returns(&[])?;
                    Ok(state)
                })
                .is_err()
        );
    }
    assert!(ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(ModifiedSharpeRatio::new(252.0, 0.0, 0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
