use super::tracking_error::TrackingError;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn computes_annualized_sample_active_return_deviation() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, 0.025, 0.01];
    let active = [0.02, 0.01, -0.005, 0.03];
    let mean = active.iter().sum::<f64>() / active.len() as f64;
    let sample_variance = active
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (active.len() - 1) as f64;
    let expected = sample_variance.sqrt() * 252.0_f64.sqrt();
    let mut state = TrackingError::new(252.0, true, NanPolicy::Omit)
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
fn supports_unannualized_output_and_pairwise_missing_omission() {
    let mut state = TrackingError::new(12.0, false, NanPolicy::Omit)
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

    let expected = 0.0002_f64.sqrt();
    assert_close(state.value().unwrap(), expected);
    assert_eq!(state.len(), 2);
}

#[test]
fn input_modes_produce_equivalent_tracking_error() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = TrackingError::new(12.0, true, NanPolicy::Omit)
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

    let mut equity = TrackingError::new(12.0, true, NanPolicy::Omit)
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

    let mut pnl = TrackingError::new(12.0, true, NanPolicy::Omit)
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

    let mut logarithmic = TrackingError::new(12.0, true, NanPolicy::Omit)
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
fn rejects_misalignment_invalid_configuration_and_domains() {
    let mut state = TrackingError::new(252.0, true, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);

    for periods_per_year in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(TrackingError::new(periods_per_year, true, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[], &[])?;
                Ok(state)
            })
            .is_err());
    }
    assert!(TrackingError::new(252.0, true, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0, 0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(TrackingError::new(252.0, true, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0, 0.0)?;
            Ok(state)
        })
        .is_err());
}
