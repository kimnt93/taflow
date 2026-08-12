use super::profit_factor::ProfitFactor;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

#[test]
fn computes_gross_positive_sum_over_absolute_negative_sum() {
    let mut state = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state.extend(&[0.10, -0.04, 0.0, 0.02, -0.01]).unwrap();
    assert_relative_eq!(state.compute().unwrap(), 2.4, epsilon = 1e-15);
}

#[test]
fn freezes_zero_denominator_edge_matrix() {
    let mut state = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.value(), None);
    state.extend(&[0.0, 0.0]).unwrap();
    assert_eq!(state.value(), None);
    state.append(0.1).unwrap();
    assert_eq!(state.value(), Some(f64::INFINITY));
    state.append(-0.05).unwrap();
    assert_relative_eq!(state.value().unwrap(), 2.0, epsilon = 1e-15);

    let mut loss_only = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    loss_only.extend(&[-0.02, -0.03]).unwrap();
    assert_eq!(loss_only.value(), Some(0.0));
}

#[test]
fn raw_pnl_and_trade_domains_preserve_monetary_observations() {
    for input_kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let mut state = ProfitFactor::new(NanPolicy::Omit).unwrap();
        match input_kind {
            MetricInputKind::RawPnl => state.from_pnl(&[]).unwrap(),
            MetricInputKind::Trades => state.from_trades(&[]).unwrap(),
            _ => unreachable!(),
        };
        state.extend(&[100.0, -40.0, 0.0, 20.0, -10.0]).unwrap();
        assert_relative_eq!(state.compute().unwrap(), 2.4, epsilon = 1e-15);
    }
}

#[test]
fn lifecycle_omission_and_reset_are_invariant() {
    let values = [0.10, f64::NAN, -0.04, 0.0, 0.02, -0.01];
    let mut batch = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 5);
    let expected = batch.value().unwrap();

    let mut streamed = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    for value in values {
        streamed.append(value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn validates_input_method_domains_and_observation_semantics() {
    assert!(ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());

    let mut returns = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(returns.append(-2.0).is_err());
    assert!(returns.is_empty());

    let mut pnl = ProfitFactor::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(pnl.append(-2.0).unwrap(), Some(0.0));
}
