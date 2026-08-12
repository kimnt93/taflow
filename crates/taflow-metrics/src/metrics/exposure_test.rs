use super::exposure::{Exposure, ExposureInputKind};
use crate::NanPolicy;
use approx::assert_relative_eq;

// QuantStats 0.0.81 `stats.exposure` source convention: count finite non-zero
// values, divide by the number of periods, then ceil to the next 0.01. TAFlow
// applies its package-wide NaN policy first, so oracle inputs are pre-filtered.
fn quantstats_0_0_81(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let exposed = values.iter().filter(|&&value| value != 0.0).count();
    Some(((exposed as f64 / values.len() as f64) * 100.0).ceil() / 100.0)
}

#[test]
fn matches_pinned_quantstats_ceiling_contract() {
    for values in [
        vec![0.01, 0.0, 0.02, 0.0, 0.03],
        vec![0.01, 0.0, 0.0],
        vec![0.0; 101],
        vec![0.01; 101],
    ] {
        let mut state = Exposure::new(NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .unwrap();
        assert_eq!(state.extend(&values).unwrap(), quantstats_0_0_81(&values));
    }

    let mut state = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_relative_eq!(
        state.extend(&[0.01, 0.0, 0.0]).unwrap().unwrap(),
        0.34,
        epsilon = 1e-15
    );
}

#[test]
fn explicit_position_state_does_not_infer_returns() {
    let positions = [0.0, 1.0, -1.0, 0.0, 0.5, 2.0];
    let mut state = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_positions(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.extend(&positions).unwrap(), Some(0.67));
    assert_eq!(state.len(), positions.len());
}

#[test]
fn scalar_chunk_reset_and_cached_compute_are_invariant() {
    let values = [0.01, f64::NAN, 0.0, -0.02, 0.0, 0.03];
    let mut batch = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let expected = batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);
    assert_eq!(batch.compute(), expected);
    assert_eq!(batch.compute(), expected);

    let mut scalar = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }
    assert_eq!(scalar.compute(), expected);
    scalar.reset();
    assert!(scalar.is_empty());
    assert_eq!(scalar.compute(), None);
    scalar.extend(&values[..3]).unwrap();
    scalar.extend(&values[3..]).unwrap();
    assert_eq!(scalar.compute(), expected);
}

#[test]
fn validates_domain_specific_observations_and_missing_policy() {
    let mut returns = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(returns.append(f64::NAN).unwrap(), None);
    assert!(returns.is_empty());
    assert!(returns.append(-1.01).is_err());
    assert!(returns.is_empty());
    assert!(returns.append(f64::INFINITY).is_err());
    assert!(returns.is_empty());

    let mut positions = Exposure::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_positions(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(positions.append(-2.0).unwrap(), Some(1.0));

    let mut strict = Exposure::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_positions(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(strict.append(f64::NAN).is_err());
    assert!(strict.is_empty());
}
