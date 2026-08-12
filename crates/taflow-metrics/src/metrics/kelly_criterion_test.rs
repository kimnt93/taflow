use super::kelly_criterion::KellyCriterion;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_binary_historical_kelly_from_decisive_observations() {
    // Wins are 0.10 and 0.30; losses are -0.20 and -0.10. Thus p=q=0.5,
    // average win=0.20, average loss=-0.15, and payoff ratio=4/3.
    let values = [0.10, 0.0, -0.20, 0.30, -0.10, 0.0];
    let payoff_ratio = 4.0 / 3.0;
    let expected = ((payoff_ratio * 0.5) - 0.5) / payoff_ratio;
    let mut state = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&values).unwrap();

    assert_close(state.value().unwrap(), expected);
    assert_eq!(state.compute(), state.value());
    assert_eq!(state.len(), values.len());
}

#[test]
fn closed_trade_values_are_consumed_without_return_conversion() {
    let trades = [100.0, -20.0, 0.0, 50.0, -10.0];
    let mut state = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&trades).unwrap();

    let payoff_ratio = 75.0 / 15.0;
    let expected = ((payoff_ratio * 0.5) - 0.5) / payoff_ratio;
    assert_close(state.value().unwrap(), expected);
}

#[test]
fn breakevens_are_excluded_from_probability_but_included_in_length() {
    let base = [0.10, -0.05];
    let with_breakevens = [0.10, 0.0, -0.05, -0.0, 0.0];
    let mut base_state = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    base_state.extend(&base).unwrap();
    let mut with_breakevens_state = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    with_breakevens_state.extend(&with_breakevens).unwrap();

    assert_eq!(base_state.value(), with_breakevens_state.value());
    assert_eq!(with_breakevens_state.len(), with_breakevens.len());
}

#[test]
fn chunking_missing_values_and_reset_are_invariant() {
    let values = [0.10, f64::NAN, 0.0, -0.20, 0.30, -0.10];
    let mut batch = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);
    let expected = batch.value().unwrap();

    let mut streamed = KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    streamed.append(values[0]).unwrap();
    streamed.extend(&values[1..4]).unwrap();
    streamed.extend(&values[4..]).unwrap();
    assert_close(streamed.compute().unwrap(), expected);

    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_close(streamed.compute().unwrap(), expected);
}

#[test]
fn undefined_edges_and_invalid_domains_are_explicit() {
    for values in [&[][..], &[0.0, -0.0], &[0.10, 0.20], &[-0.10, -0.20]] {
        let mut state = KellyCriterion::new(NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[])?;
                Ok(state)
            })
            .unwrap();
        state.extend(values).unwrap();
        assert_eq!(state.value(), None);
    }

    assert!(KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(KellyCriterion::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut state = KellyCriterion::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}
