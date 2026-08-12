use super::EntropicValueAtRisk;
use crate::{MetricInputKind, NanPolicy};

// Definition decision: this follows Riskfolio-Lib 7.3 development source at
// commit 632a9e48fbaf2b9f8e83864a492332364b6ed32c (RiskFunctions.EVaR_Hist),
// SHA-256 eed72dceb7024c9ead811fa12d8e834618604f8db9c4fbb6367ce4d2a3992719.
// It is empirical positive-loss EVaR at lower-tail alpha, not a parametric
// distribution fit. TAFlow replaces solver fallback ambiguity with stable
// shifted log-sum-exp and deterministic first-order-condition bisection.

#[test]
fn matches_independently_evaluated_empirical_objective() {
    let returns: Vec<f64> = (0..101)
        .map(|index| -0.04 + index as f64 * 0.0009)
        .collect();
    let mut metric =
        EntropicValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    metric.extend(&returns).unwrap();
    let actual = metric.compute().unwrap();

    // Reference optimum and objective computed independently with SciPy 1.18
    // minimize_scalar on Riskfolio-Lib's published scalar formula.
    let expected = 0.038_757_728_645_591_79;
    assert!((actual - expected).abs() < 2e-12, "{actual} != {expected}");
}

#[test]
fn finite_sample_boundary_and_constant_follow_worst_loss() {
    let mut short =
        EntropicValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    short.extend(&[0.02, -0.01, -0.04, 0.03]).unwrap();
    assert_eq!(short.compute(), Some(0.04));

    let mut constant =
        EntropicValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    constant.extend(&[0.0125; 64]).unwrap();
    assert_eq!(constant.compute(), Some(-0.0125));
}

#[test]
fn factories_lifecycle_and_lazy_cache_are_invariant() {
    let returns: Vec<f64> = (0..64)
        .map(|index| ((index * 37 % 101) as f64 - 50.0) / 2_000.0)
        .collect();
    let log_returns: Vec<f64> = returns.iter().map(|value| value.ln_1p()).collect();
    let mut equity = vec![100.0];
    for &value in &returns {
        equity.push(equity.last().unwrap() * (1.0 + value));
    }
    let pnl: Vec<f64> = equity.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let create = |kind| EntropicValueAtRisk::new(kind, 0.10, NanPolicy::Omit).unwrap();

    let mut from_returns = create(MetricInputKind::Returns);
    from_returns.extend(&returns).unwrap();
    let expected = from_returns.compute().unwrap();
    assert_eq!(from_returns.compute(), Some(expected));

    let mut from_logs = create(MetricInputKind::LogReturns);
    let mut from_equity = create(MetricInputKind::Equity);
    let mut from_pnl = create(MetricInputKind::PeriodPnl {
        initial_equity: 100.0,
    });
    from_logs.extend(&log_returns).unwrap();
    from_equity.extend(&equity).unwrap();
    from_pnl.extend(&pnl).unwrap();
    assert!((from_logs.compute().unwrap() - expected).abs() < 1e-14);
    assert!((from_equity.compute().unwrap() - expected).abs() < 1e-14);
    assert!((from_pnl.compute().unwrap() - expected).abs() < 1e-14);

    from_returns.append(-0.03).unwrap();
    let continued = from_returns.compute();
    let mut replay = create(MetricInputKind::Returns);
    replay.extend(&returns).unwrap();
    replay.append(-0.03).unwrap();
    assert_eq!(continued, replay.compute());

    from_returns.reset();
    assert!(from_returns.is_empty());
    assert_eq!(from_returns.compute(), None);
}

#[test]
fn validates_cutoff_domain_and_missing_policy() {
    for cutoff in [f64::NAN, f64::NEG_INFINITY, 0.0, 1.0, f64::INFINITY] {
        assert!(
            EntropicValueAtRisk::new(MetricInputKind::Returns, cutoff, NanPolicy::Omit).is_err()
        );
    }
    assert!(EntropicValueAtRisk::new(MetricInputKind::RawPnl, 0.05, NanPolicy::Omit).is_err());
    assert!(EntropicValueAtRisk::new(MetricInputKind::Trades, 0.05, NanPolicy::Omit).is_err());

    let mut omit =
        EntropicValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    omit.extend(&[f64::NAN, 0.01, -0.02]).unwrap();
    assert_eq!(omit.len(), 2);
    let mut raise =
        EntropicValueAtRisk::new(MetricInputKind::Returns, 0.05, NanPolicy::Raise).unwrap();
    assert!(raise.append(f64::NAN).is_err());
}
