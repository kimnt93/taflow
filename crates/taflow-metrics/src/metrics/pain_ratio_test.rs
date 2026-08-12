use super::pain_ratio::PainRatio;
use crate::{MetricInputKind, NanPolicy};

fn expected_performanceanalytics_definition(
    returns: &[f64],
    periods_per_year: f64,
    annual_risk_free_rate: f64,
) -> Option<f64> {
    // PerformanceAnalytics 2.1.0 PainRatio.R computes geometric CAGR, then
    // subtracts Rf, and divides by PainIndex. The PainIndex source averages
    // absolute DrawdownPeak values over the usable observations.
    if returns.is_empty() {
        return None;
    }
    let mut wealth = 1.0_f64;
    let mut peak = 1.0_f64;
    let mut pain_sum = 0.0_f64;
    for &value in returns {
        wealth *= 1.0 + value;
        peak = peak.max(wealth);
        pain_sum += (wealth / peak - 1.0).abs();
    }
    let pain_index = pain_sum / returns.len() as f64;
    if pain_index == 0.0 {
        return None;
    }
    let annualized_return = wealth.powf(periods_per_year / returns.len() as f64) - 1.0;
    Some((annualized_return - annual_risk_free_rate) / pain_index)
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_pinned_source_definition_and_preserves_lifecycle() {
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01];
    let expected = expected_performanceanalytics_definition(&returns, 12.0, 0.03).unwrap();
    let mut state = PainRatio::new(12.0, 0.03, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    state.append(returns[0]).unwrap();
    assert_eq!(state.value(), None);
    state.extend(&returns[1..3]).unwrap();
    state.append(returns[3]).unwrap();
    assert_close(state.append(returns[4]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), returns.len());
    assert_eq!(state.periods_per_year(), 12.0);
    assert_eq!(state.annual_risk_free_rate(), 0.03);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn all_return_input_modes_are_equivalent() {
    let returns = [0.10, -0.20, 0.05];
    let expected = expected_performanceanalytics_definition(&returns, 12.0, 0.02).unwrap();

    let mut equity = PainRatio::new(12.0, 0.02, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );

    let mut pnl = PainRatio::new(12.0, 0.02, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.0)?;
            Ok(state)
        })
        .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let logarithmic_returns = returns.map(f64::ln_1p);
    let mut logarithmic = PainRatio::new(12.0, 0.02, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        logarithmic.extend(&logarithmic_returns).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn handles_missing_values_zero_pain_and_total_loss() {
    let mut state = PainRatio::new(12.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[f64::NAN, 0.10, -0.20]).unwrap();
    assert_eq!(state.len(), 2);
    assert_close(
        state.value().unwrap(),
        expected_performanceanalytics_definition(&[0.10, -0.20], 12.0, 0.0).unwrap(),
    );

    let mut no_pain = PainRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    no_pain.extend(&[0.01, 0.02]).unwrap();
    assert_eq!(no_pain.value(), None);

    let mut total_loss = PainRatio::new(1.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(total_loss.append(-1.0).unwrap(), Some(-1.0));
}

#[test]
fn raise_policy_rejects_nan_without_mutating_state() {
    let mut state = PainRatio::new(12.0, 0.0, NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.append(-0.01).unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert_eq!(state.len(), 1);
    assert!(state.value().is_some());
}

#[test]
fn validates_configuration_values_and_semantic_domain() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(PainRatio::new(invalid, 0.0, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .is_err());
    }
    for invalid in [-1.0, -2.0, f64::NAN, f64::INFINITY] {
        assert!(PainRatio::new(252.0, invalid, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .is_err());
    }
    assert!(PainRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(PainRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = PainRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
}
