use super::ParametricExpectedShortfall;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn computes_gaussian_lower_tail_mean() {
    let mut m = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let actual = m
        .extend(&[-0.02, 0.01, 0.03, -0.01, 0.015])
        .unwrap()
        .unwrap();
    assert!((actual - (-0.036_254_256_150_148_51)).abs() < 2e-10);
}
#[test]
fn input_methods_lifecycle_and_missing_are_invariant() {
    let returns: [f64; 3] = [0.01, -0.02, 0.03];
    let logs: Vec<_> = returns.iter().map(|v| v.ln_1p()).collect();
    let equity = [100., 101., 98.98, 101.9494];
    let pnl = [1., -2.02, 2.9694];
    let mut a = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut b = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut c = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut d = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.)?;
            Ok(state)
        })
        .unwrap();
    let expected = a.extend(&returns).unwrap().unwrap();
    assert!((b.extend(&logs).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((c.extend(&equity).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((d.extend(&pnl).unwrap().unwrap() - expected).abs() < 1e-14);
    a.reset();
    assert_eq!(
        a.extend(&[returns[0], f64::NAN, returns[1], returns[2]])
            .unwrap(),
        Some(expected)
    );
}
#[test]
fn validates_cutoff_domain_and_warmup() {
    assert!(ParametricExpectedShortfall::new(0., NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .is_err());
    assert!(ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    let mut m = ParametricExpectedShortfall::new(0.05, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(m.append(0.01).unwrap(), None);
    assert_eq!(m.append(0.01).unwrap(), Some(0.01));
}
