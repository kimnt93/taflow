use super::probabilistic_sharpe_ratio::ProbabilisticSharpeRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn matches_pinned_vectorbt_formula_with_bias_corrected_sample_moments() {
    // vectorbt 0.28.5 commit 993ceca7116fc8e55f4cd3a36fe43d83dab62b27,
    // returns/metrics.py::deflated_sharpe_ratio. TAFlow substitutes a caller-
    // supplied benchmark Sharpe for vectorbt's estimated maximum Sharpe and
    // freezes scipy.stats skew/kurtosis with bias=False. The expected value was
    // evaluated independently with SciPy 1.17.0 norm.cdf/skew/kurtosis.
    let returns = [0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.015];
    let mut state = ProbabilisticSharpeRatio::new(252.0, 0.0, 0.5, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.extend(&returns[..3]).unwrap(), None);
    state.extend(&returns[3..]).unwrap();
    assert_close(state.compute().unwrap(), 0.469_251_442_389_235_66, 2e-12);
    assert_eq!(state.len(), returns.len());
}

#[test]
fn semantic_input_modes_and_streaming_lifecycle_are_invariant() {
    let returns = [0.10, -0.20, 0.05, -0.03, 0.08, -0.01];
    let make = |kind| {
        let mut state = ProbabilisticSharpeRatio::new(12.0, 0.03, 0.4, NanPolicy::Omit).unwrap();
        match kind {
            MetricInputKind::Returns => state.from_returns(&[]).unwrap(),
            MetricInputKind::LogReturns => state.from_log_returns(&[]).unwrap(),
            MetricInputKind::Equity => state.from_equity(&[]).unwrap(),
            MetricInputKind::PeriodPnl { initial_capital } => {
                state.from_pnl(&[], initial_capital).unwrap()
            }
            _ => unreachable!(),
        };
        state
    };
    let mut direct = make(MetricInputKind::Returns);
    let expected = direct.extend(&returns).unwrap().unwrap();

    let mut equity = make(MetricInputKind::Equity);
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4, 89.628, 96.798_24, 95.830_257_6])
            .unwrap()
            .unwrap(),
        expected,
        2e-12,
    );
    let mut pnl = make(MetricInputKind::PeriodPnl {
        initial_capital: 100.0,
    });
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4, -2.772, 7.170_24, -0.967_982_4])
            .unwrap()
            .unwrap(),
        expected,
        2e-12,
    );
    let mut logarithmic = make(MetricInputKind::LogReturns);
    assert_close(
        logarithmic
            .extend(&returns.map(f64::ln_1p))
            .unwrap()
            .unwrap(),
        expected,
        2e-12,
    );

    direct.reset();
    assert!(direct.is_empty());
    assert_eq!(direct.value(), None);
    for &value in &returns[..3] {
        assert_eq!(direct.append(value).unwrap(), None);
    }
    direct.extend(&returns[3..]).unwrap();
    assert_close(direct.compute().unwrap(), expected, 2e-12);
}

#[test]
fn handles_small_samples_constant_data_and_missing_values() {
    let mut state = ProbabilisticSharpeRatio::new(252.0, 0.0, 0.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[f64::NAN, 0.01, -0.02, 0.03]).unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state.compute(), None);
    state.append(-0.01).unwrap();
    assert!(state.compute().is_some());

    state.reset();
    state.extend(&[0.01; 8]).unwrap();
    assert_eq!(state.compute(), None);

    let mut raise = ProbabilisticSharpeRatio::new(252.0, 0.0, 0.0, NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert!(raise.is_empty());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}

#[test]
fn validates_configuration_and_semantic_domain() {
    for periods in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            ProbabilisticSharpeRatio::new(periods, 0.0, 0.0, NanPolicy::Omit)
                .and_then(|mut state| {
                    state.from_returns(&[])?;
                    Ok(state)
                })
                .is_err()
        );
    }
    for rate in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(
            ProbabilisticSharpeRatio::new(252.0, rate, 0.0, NanPolicy::Omit)
                .and_then(|mut state| {
                    state.from_returns(&[])?;
                    Ok(state)
                })
                .is_err()
        );
    }
    for benchmark in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            ProbabilisticSharpeRatio::new(252.0, 0.0, benchmark, NanPolicy::Omit)
                .and_then(|mut state| {
                    state.from_returns(&[])?;
                    Ok(state)
                })
                .is_err()
        );
    }
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let _ = kind;
        let mut state = ProbabilisticSharpeRatio::new(252.0, 0.0, 0.0, NanPolicy::Omit).unwrap();
        assert!(state.append(0.0).is_err());
    }
}
