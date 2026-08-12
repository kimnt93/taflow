use super::treynor_ratio::TreynorRatio;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

// PerformanceAnalytics 2.1.0 source tarball SHA-256:
// fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d.
fn source_convention(
    primary: &[f64],
    benchmark: &[f64],
    periods_per_year: f64,
    annual_risk_free_rate: f64,
) -> f64 {
    let risk_free = (annual_risk_free_rate.ln_1p() / periods_per_year).exp_m1();
    let primary_excess: Vec<_> = primary.iter().map(|value| value - risk_free).collect();
    let benchmark_excess: Vec<_> = benchmark.iter().map(|value| value - risk_free).collect();
    let count = primary.len() as f64;
    let primary_mean = primary_excess.iter().sum::<f64>() / count;
    let benchmark_mean = benchmark_excess.iter().sum::<f64>() / count;
    let covariance = primary_excess
        .iter()
        .zip(&benchmark_excess)
        .map(|(primary, benchmark)| (primary - primary_mean) * (benchmark - benchmark_mean))
        .sum::<f64>()
        / (count - 1.0);
    let benchmark_variance = benchmark_excess
        .iter()
        .map(|benchmark| (benchmark - benchmark_mean).powi(2))
        .sum::<f64>()
        / (count - 1.0);
    let beta = covariance / benchmark_variance;
    let growth = primary_excess
        .iter()
        .map(|value| 1.0 + value)
        .product::<f64>();
    (growth.powf(periods_per_year / count) - 1.0) / beta
}

#[test]
fn matches_pinned_performanceanalytics_source_convention() {
    let primary = [0.08, -0.03, 0.04, 0.01, -0.02];
    let benchmark = [0.05, -0.02, 0.02, 0.00, -0.01];
    let periods_per_year = 12.0;
    let annual_risk_free_rate = 0.061_677_811_864_498_28;
    let mut state = TreynorRatio::new(periods_per_year, annual_risk_free_rate, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&primary, &benchmark).unwrap();
    assert_relative_eq!(
        state.compute().unwrap(),
        source_convention(
            &primary,
            &benchmark,
            periods_per_year,
            annual_risk_free_rate
        ),
        epsilon = 1e-13
    );
}

#[test]
fn lifecycle_pairwise_omission_and_reset_are_invariant() {
    let primary = [0.08, f64::NAN, -0.03, 0.04, 0.01];
    let benchmark = [0.05, 0.03, -0.02, 0.02, 0.00];
    let mut batch = TreynorRatio::new(12.0, 0.03, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&primary, &benchmark).unwrap();
    assert_eq!(batch.len(), 4);
    let expected = batch.value().unwrap();

    let mut streamed = TreynorRatio::new(12.0, 0.03, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    for (&value, &benchmark_value) in primary.iter().zip(&benchmark) {
        streamed.append(value, benchmark_value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&primary, &benchmark).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn requires_two_pairs_and_nonzero_beta() {
    let mut state = TreynorRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    state.append(0.01, 0.02).unwrap();
    assert_eq!(state.value(), None);
    state.append(0.02, 0.04).unwrap();
    assert!(state.value().is_some());

    let mut zero_beta = TreynorRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    zero_beta.extend(&[0.01, 0.01], &[0.00, 0.02]).unwrap();
    assert_eq!(zero_beta.value(), None);
}

#[test]
fn rejects_invalid_configuration_and_domains() {
    assert!(TreynorRatio::new(0.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .is_err());
    assert!(TreynorRatio::new(252.0, -1.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .is_err());
    assert!(TreynorRatio::new(252.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0, 0.0)?;
            Ok(state)
        })
        .is_err());
}
