use super::beta::Beta;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn computes_sample_covariance_over_benchmark_variance() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, 0.025, 0.01];
    let primary_mean = primary.iter().sum::<f64>() / primary.len() as f64;
    let benchmark_mean = benchmark.iter().sum::<f64>() / benchmark.len() as f64;
    let covariance = primary
        .iter()
        .zip(benchmark)
        .map(|(primary, benchmark)| (primary - primary_mean) * (benchmark - benchmark_mean))
        .sum::<f64>()
        / (primary.len() - 1) as f64;
    let benchmark_variance = benchmark
        .iter()
        .map(|benchmark| (benchmark - benchmark_mean).powi(2))
        .sum::<f64>()
        / (benchmark.len() - 1) as f64;
    let expected = covariance / benchmark_variance;
    let mut state = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(primary[0], benchmark[0]).unwrap(), None);
    state.extend(&primary[1..3], &benchmark[1..3]).unwrap();
    assert_close(
        state.append(primary[3], benchmark[3]).unwrap().unwrap(),
        expected,
    );
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), primary.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(
        state.extend(&primary, &benchmark).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn omits_missing_values_pairwise_and_handles_undefined_cases() {
    let mut state = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    state
        .extend(
            &[0.01, f64::NAN, 0.05, -0.02],
            &[0.00, 0.02, f64::NAN, -0.01],
        )
        .unwrap();

    assert_close(state.value().unwrap(), 3.0);
    assert_eq!(state.len(), 2);

    let mut constant_benchmark = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    constant_benchmark
        .extend(&[0.01, 0.02, 0.03], &[0.01, 0.01, 0.01])
        .unwrap();
    assert_eq!(constant_benchmark.value(), None);
}

#[test]
fn input_modes_produce_equivalent_beta() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = Beta::new(NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[], &[])?;
                Ok(state)
            })
            .unwrap();
        state
            .extend(&primary_returns, &benchmark_returns)
            .unwrap()
            .unwrap()
    };

    let mut equity = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4], &[200.0, 204.0, 183.6, 185.436])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut pnl = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], &[], 100.0, 200.0)?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    let primary_log_returns = primary_returns.map(f64::ln_1p);
    let benchmark_log_returns = benchmark_returns.map(f64::ln_1p);
    assert_close(
        logarithmic
            .extend(&primary_log_returns, &benchmark_log_returns)
            .unwrap()
            .unwrap(),
        expected,
    );
}

#[test]
fn rejects_misalignment_invalid_domains_and_nan_when_requested() {
    let mut state = Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);

    assert!(Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0, 0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(Beta::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0, 0.0)?;
            Ok(state)
        })
        .is_err());

    let mut raises = Beta::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert!(raises.append(f64::NAN, 0.01).is_err());
    assert_eq!(raises.len(), 0);
}
