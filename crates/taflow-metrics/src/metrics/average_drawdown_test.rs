use super::average_drawdown::AverageDrawdown;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

// PerformanceAnalytics 2.1.0 source tarball SHA-256:
// fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d.
fn source_convention(returns: &[f64]) -> f64 {
    let mut wealth = 1.0_f64;
    let mut peak = 1.0_f64;
    let mut episode_depths = Vec::new();
    let mut current_depth: Option<f64> = None;
    for &simple_return in returns {
        wealth *= 1.0 + simple_return;
        peak = peak.max(wealth);
        let drawdown = wealth / peak - 1.0;
        if drawdown < 0.0 {
            current_depth = Some(current_depth.map_or(drawdown, |depth| depth.min(drawdown)));
        } else if let Some(depth) = current_depth.take() {
            episode_depths.push(depth.abs());
        }
    }
    if let Some(depth) = current_depth {
        episode_depths.push(depth.abs());
    }
    if episode_depths.is_empty() {
        0.0
    } else {
        episode_depths.iter().sum::<f64>() / episode_depths.len() as f64
    }
}

#[test]
fn matches_pinned_performanceanalytics_episode_convention() {
    let returns = [0.25, -0.20, 0.0, 0.10, -0.50, 1.0, -0.10];
    let mut state = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&returns).unwrap();
    assert_relative_eq!(
        state.compute().unwrap(),
        source_convention(&returns),
        epsilon = 1e-15
    );
}

#[test]
fn completed_and_current_episodes_are_counted_once() {
    let mut state = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[0.25, -0.20]).unwrap();
    assert_relative_eq!(state.value().unwrap(), 0.20, epsilon = 1e-15);
    state.append(0.25).unwrap();
    assert_relative_eq!(state.value().unwrap(), 0.20, epsilon = 1e-15);
    state.extend(&[0.10, -0.50]).unwrap();
    assert_relative_eq!(state.value().unwrap(), 0.35, epsilon = 1e-15);
}

#[test]
fn nonempty_path_without_drawdowns_returns_zero() {
    let mut state = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.value(), None);
    state.extend(&[0.10, 0.0, 0.20]).unwrap();
    assert_eq!(state.value(), Some(0.0));
}

#[test]
fn lifecycle_omission_and_reset_are_invariant() {
    let returns = [0.25, f64::NAN, -0.20, 0.25, 0.10, -0.50, 1.0, -0.10];
    let mut batch = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&returns).unwrap();
    assert_eq!(batch.len(), 7);
    let expected = batch.value().unwrap();

    let mut streamed = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    for value in returns {
        streamed.append(value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&returns).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn rejects_non_path_domains_and_invalid_observations() {
    assert!(AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    let mut state = AverageDrawdown::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.is_empty());
}
