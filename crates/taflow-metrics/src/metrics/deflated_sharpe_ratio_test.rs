use super::deflated_sharpe_ratio::DeflatedSharpeRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn matches_pinned_vectorbt_deflated_sharpe_formula() {
    // vectorbt 0.28.5 commit 993ceca7116fc8e55f4cd3a36fe43d83dab62b27.
    // Expected value independently evaluated with SciPy 1.17.0.
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.015];
    let mut state = DeflatedSharpeRatio::new(
        MetricInputKind::Returns,
        252.0,
        0.0,
        20,
        0.64,
        NanPolicy::Omit,
    )
    .unwrap();
    state.extend(&returns).unwrap();
    assert_close(state.compute().unwrap(), 0.407_248_550_042_508_76, 2e-10);
}

#[test]
fn semantic_modes_chunking_and_reset_are_invariant() {
    let returns = [0.10, -0.20, 0.05, -0.03, 0.08, -0.01];
    let make = |kind| DeflatedSharpeRatio::new(kind, 12.0, 0.03, 8, 0.25, NanPolicy::Omit).unwrap();
    let mut direct = make(MetricInputKind::Returns);
    let expected = direct.extend(&returns).unwrap().unwrap();
    let mut equity = make(MetricInputKind::Equity);
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4, 89.628, 96.79824, 95.8302576])
            .unwrap()
            .unwrap(),
        expected,
        2e-10,
    );
    let mut pnl = make(MetricInputKind::PeriodPnl {
        initial_equity: 100.0,
    });
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4, -2.772, 7.17024, -0.9679824])
            .unwrap()
            .unwrap(),
        expected,
        2e-10,
    );
    let mut logarithmic = make(MetricInputKind::LogReturns);
    assert_close(
        logarithmic
            .extend(&returns.map(f64::ln_1p))
            .unwrap()
            .unwrap(),
        expected,
        2e-10,
    );
    direct.reset();
    assert!(direct.is_empty());
    for &value in &returns {
        direct.append(value).unwrap();
    }
    assert_close(direct.compute().unwrap(), expected, 2e-10);
}

#[test]
fn edges_and_validation_are_explicit() {
    let mut state = DeflatedSharpeRatio::new(
        MetricInputKind::Returns,
        252.0,
        0.0,
        2,
        0.0,
        NanPolicy::Omit,
    )
    .unwrap();
    state.extend(&[f64::NAN, 0.01, -0.02, 0.03]).unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state.compute(), None);
    state.append(-0.01).unwrap();
    assert!(state.compute().is_some());
    state.reset();
    state.extend(&[0.01; 8]).unwrap();
    assert_eq!(state.compute(), None);
    assert!(DeflatedSharpeRatio::new(
        MetricInputKind::Returns,
        252.0,
        0.0,
        1,
        0.2,
        NanPolicy::Omit
    )
    .is_err());
    assert!(DeflatedSharpeRatio::new(
        MetricInputKind::Returns,
        252.0,
        0.0,
        2,
        -0.1,
        NanPolicy::Omit
    )
    .is_err());
    assert!(
        DeflatedSharpeRatio::new(MetricInputKind::RawPnl, 252.0, 0.0, 2, 0.1, NanPolicy::Omit)
            .is_err()
    );
}
