use super::conditional_drawdown_at_risk::ConditionalDrawdownAtRisk;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

// PerformanceAnalytics 2.1.0 CRAN source tarball SHA-256:
// fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d.
// Its default discrete method applies R's type-7 quantile to the signed
// findDrawdowns episode troughs, selects every trough <= that boundary, and
// negates their mean. Riskfolio-Lib 7.3.0 CDaR_Rel is deliberately not this
// oracle: it uses every per-observation drawdown and fractional tail weighting.
fn pinned_performanceanalytics_discrete(returns: &[f64], confidence: f64) -> Option<f64> {
    if returns.is_empty() {
        return None;
    }
    let mut wealth = 1.0_f64;
    let mut peak = 1.0_f64;
    let mut episodes = Vec::new();
    let mut current: Option<(bool, f64)> = None;
    for &simple_return in returns {
        wealth *= 1.0 + simple_return;
        peak = peak.max(wealth);
        let drawdown = wealth / peak - 1.0;
        let negative = drawdown < 0.0;
        match current {
            None => current = Some((negative, drawdown)),
            Some((sign, trough)) if sign == negative => {
                current = Some((sign, trough.min(drawdown)));
            }
            Some((_, trough)) => {
                episodes.push(trough);
                current = Some((negative, drawdown));
            }
        }
    }
    episodes.push(current.unwrap().1);
    episodes.sort_by(f64::total_cmp);
    let index = (episodes.len() - 1) as f64 * (1.0 - confidence);
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    let weight = index - lower as f64;
    let boundary = episodes[lower] + (episodes[upper] - episodes[lower]) * weight;
    let selected: Vec<_> = episodes
        .into_iter()
        .take_while(|x| *x <= boundary)
        .collect();
    Some(-selected.iter().sum::<f64>() / selected.len() as f64)
}

#[test]
fn matches_pinned_performanceanalytics_discrete_episode_estimator() {
    let returns = [0.25, -0.20, 0.25, 0.10, -0.50, 1.0, -0.10];
    for confidence in [0.5, 0.75, 0.90, 0.95, 0.99] {
        let mut state = ConditionalDrawdownAtRisk::new(confidence, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .unwrap();
        let actual = state.extend(&returns).unwrap().unwrap();
        let expected = pinned_performanceanalytics_discrete(&returns, confidence).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1e-15);
    }
}

#[test]
fn includes_current_episode_and_all_boundary_ties() {
    let mut state = ConditionalDrawdownAtRisk::new(0.5, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[0.25, -0.20]).unwrap();
    assert_relative_eq!(state.compute().unwrap(), 0.20, epsilon = 1e-15);
    state.append(0.25).unwrap();
    state.extend(&[0.10, -0.50]).unwrap();
    assert_relative_eq!(state.compute().unwrap(), 0.35, epsilon = 1e-15);

    let tied = [0.25, -0.20, 0.25, 0.25, -0.20];
    let mut state = ConditionalDrawdownAtRisk::new(0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_relative_eq!(state.extend(&tied).unwrap().unwrap(), 0.20, epsilon = 1e-15);
}

#[test]
fn bulk_scalar_chunk_reset_and_cached_compute_are_invariant() {
    let values = [0.25, f64::NAN, -0.20, 0.25, 0.10, -0.50, 1.0, -0.10];
    let mut batch = ConditionalDrawdownAtRisk::new(0.75, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let expected = batch.extend(&values).unwrap().unwrap();
    assert_eq!(batch.len(), 7);
    assert_eq!(batch.compute(), Some(expected));
    assert_eq!(batch.compute(), Some(expected));

    let mut scalar = ConditionalDrawdownAtRisk::new(0.75, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }
    assert_eq!(scalar.compute(), Some(expected));

    scalar.reset();
    assert!(scalar.is_empty());
    assert_eq!(scalar.compute(), None);
    scalar.extend(&values[..4]).unwrap();
    scalar.extend(&values[4..]).unwrap();
    assert_eq!(scalar.compute(), Some(expected));
}

#[test]
fn semantic_inputs_match_returns() {
    let returns = [0.10_f64, -0.20, 0.05];
    let logs: Vec<_> = returns.iter().map(|value| value.ln_1p()).collect();
    let equity = [100.0, 110.0, 88.0, 92.4];
    let pnl = [10.0, -22.0, 4.4];

    let compute = |kind, values: &[f64]| {
        let mut state = ConditionalDrawdownAtRisk::new(0.95, NanPolicy::Omit)
            .and_then(|mut state| {
                match kind {
                    MetricInputKind::Returns => {
                        state.from_returns(&[])?;
                    }
                    MetricInputKind::LogReturns => {
                        state.from_log_returns(&[])?;
                    }
                    MetricInputKind::Equity => {
                        state.from_equity(&[])?;
                    }
                    MetricInputKind::PeriodPnl { initial_capital } => {
                        state.from_pnl(&[], initial_capital)?;
                    }
                    MetricInputKind::RawPnl | MetricInputKind::Trades => {
                        state.append(0.0)?;
                    }
                }
                Ok(state)
            })
            .unwrap();
        state.extend(values).unwrap().unwrap()
    };
    let expected = compute(MetricInputKind::Returns, &returns);
    assert_relative_eq!(
        compute(MetricInputKind::LogReturns, &logs),
        expected,
        epsilon = 1e-15
    );
    assert_relative_eq!(
        compute(MetricInputKind::Equity, &equity),
        expected,
        epsilon = 1e-15
    );
    assert_relative_eq!(
        compute(
            MetricInputKind::PeriodPnl {
                initial_capital: 100.0,
            },
            &pnl,
        ),
        expected,
        epsilon = 1e-15
    );
}

#[test]
fn validates_configuration_domains_and_observations() {
    for confidence in [0.0, 1.0, -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(ConditionalDrawdownAtRisk::new(confidence, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .is_err());
    }
    assert!(ConditionalDrawdownAtRisk::new(0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(ConditionalDrawdownAtRisk::new(0.95, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = ConditionalDrawdownAtRisk::new(0.95, NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.is_empty());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.is_empty());
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
}
