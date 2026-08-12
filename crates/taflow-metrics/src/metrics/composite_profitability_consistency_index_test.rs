use super::CompositeProfitabilityConsistencyIndex;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn computes_three_component_product() {
    let mut m = CompositeProfitabilityConsistencyIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let actual = m.extend(&[0.1, -0.05, 0.2, -0.1, 0.0]).unwrap().unwrap();
    let pf = 0.3 / 0.15;
    let wr = 0.5;
    let payoff = 0.15 / 0.075;
    assert!((actual - pf * wr * payoff).abs() < 1e-14);
    assert_eq!(m.len(), 5);
}
#[test]
fn trades_and_returns_preserve_values() {
    let values = [0.1, -0.2, 0.3];
    let mut a = CompositeProfitabilityConsistencyIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    let mut b = CompositeProfitabilityConsistencyIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(a.extend(&values).unwrap(), b.extend(&values).unwrap());
}
#[test]
fn edges_lifecycle_and_validation() {
    let mut m = CompositeProfitabilityConsistencyIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(m.extend(&[1.0, 0.0, f64::NAN]).unwrap(), None);
    m.append(-1.0).unwrap();
    assert!(m.value().is_some());
    m.reset();
    assert_eq!(m.value(), None);
    assert!(CompositeProfitabilityConsistencyIndex::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
