use super::MaximumDrawdownDuration;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn counts_performanceanalytics_drawdown_episode_lengths() {
    let mut metric = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(metric.extend(&[0.1, -0.1]).unwrap(), Some(2));
    assert_eq!(metric.append(0.2).unwrap(), Some(2));
    assert_eq!(metric.extend(&[-0.1, -0.1, -0.1]).unwrap(), Some(4));
}

#[test]
fn non_drawdown_path_is_undefined_and_nan_is_omitted() {
    let mut metric = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(metric.extend(&[0.1, f64::NAN, 0.0, 0.2]).unwrap(), None);
    assert_eq!(metric.len(), 3);
}

#[test]
fn input_methods_and_lifecycle_are_invariant() {
    let returns: [f64; 3] = [0.1, -0.2, 0.05];
    let logs: Vec<_> = returns.iter().map(|value| value.ln_1p()).collect();
    let equity = [100.0, 110.0, 88.0, 92.4];
    let pnl = [10.0, -22.0, 4.4];
    let mut a = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut b = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut c = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut d = MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], 100.0)?;
            Ok(state)
        })
        .unwrap();
    let expected = a.extend(&returns).unwrap();
    assert_eq!(b.extend(&logs).unwrap(), expected);
    assert_eq!(c.extend(&equity).unwrap(), expected);
    assert_eq!(d.extend(&pnl).unwrap(), expected);
    a.reset();
    assert_eq!(a.extend(&returns).unwrap(), expected);
}

#[test]
fn rejects_non_path_domains() {
    assert!(MaximumDrawdownDuration::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
