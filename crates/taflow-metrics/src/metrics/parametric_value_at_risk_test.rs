use super::ParametricValueAtRisk;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn computes_signed_gaussian_lower_quantile() {
    let returns = [-0.02, 0.01, 0.03, -0.01, 0.015];
    let mut metric =
        ParametricValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();

    let actual = metric.extend(&returns).unwrap().unwrap();
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (returns.len() - 1) as f64;
    let expected = mean - 1.644_853_626_951_472_2 * variance.sqrt();

    assert!((actual - expected).abs() < 2e-10);
}

#[test]
fn factories_and_lifecycle_are_invariant() {
    let returns: [f64; 4] = [0.10, -0.20, 0.05, -0.03];
    let log_returns: Vec<f64> = returns.iter().map(|value| value.ln_1p()).collect();
    let equity = [100.0, 110.0, 88.0, 92.4, 89.628];
    let pnl = [10.0, -22.0, 4.4, -2.772];
    let create = |kind| ParametricValueAtRisk::new(kind, 0.05, NanPolicy::Omit).unwrap();

    let mut from_returns = create(MetricInputKind::Returns);
    let expected = from_returns.extend(&returns).unwrap().unwrap();
    let mut from_logs = create(MetricInputKind::LogReturns);
    let mut from_equity = create(MetricInputKind::Equity);
    let mut from_pnl = create(MetricInputKind::PeriodPnl {
        initial_equity: 100.0,
    });

    assert!((from_logs.extend(&log_returns).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((from_equity.extend(&equity).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((from_pnl.extend(&pnl).unwrap().unwrap() - expected).abs() < 1e-14);

    from_returns.reset();
    assert!(from_returns.is_empty());
    assert_eq!(from_returns.append(returns[0]).unwrap(), None);
    assert_eq!(from_returns.len(), 1);
    from_returns.extend(&returns[1..]).unwrap();
    assert_eq!(from_returns.compute(), Some(expected));

    let before_continuation = from_returns.compute();
    assert_eq!(from_returns.compute(), before_continuation);
    from_returns.append(0.02).unwrap();
    let mut replay = create(MetricInputKind::Returns);
    replay.extend(&[0.10, -0.20, 0.05, -0.03, 0.02]).unwrap();
    assert_eq!(from_returns.compute(), replay.compute());
}

#[test]
fn omits_missing_values_and_constant_sample_returns_mean() {
    let mut with_missing =
        ParametricValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    let mut without_missing =
        ParametricValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    assert_eq!(
        with_missing.extend(&[0.01, f64::NAN, -0.02, 0.03]).unwrap(),
        without_missing.extend(&[0.01, -0.02, 0.03]).unwrap()
    );
    assert_eq!(with_missing.len(), 3);

    let mut constant =
        ParametricValueAtRisk::new(MetricInputKind::Returns, 0.01, NanPolicy::Omit).unwrap();
    assert_eq!(constant.extend(&[0.0125, 0.0125]).unwrap(), Some(0.0125));
}

#[test]
fn validates_configuration_and_requires_two_returns() {
    for cutoff in [f64::NAN, f64::NEG_INFINITY, 0.0, 1.0, f64::INFINITY] {
        assert!(
            ParametricValueAtRisk::new(MetricInputKind::Returns, cutoff, NanPolicy::Omit).is_err()
        );
    }
    assert!(ParametricValueAtRisk::new(MetricInputKind::RawPnl, 0.05, NanPolicy::Omit).is_err());
    assert!(ParametricValueAtRisk::new(MetricInputKind::Trades, 0.05, NanPolicy::Omit).is_err());

    let mut metric =
        ParametricValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    assert_eq!(metric.compute(), None);
    assert_eq!(metric.append(0.01).unwrap(), None);
    assert!(metric.append(0.02).unwrap().is_some());
}
