use super::BreakevenRate;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn counts_exact_zero_across_supported_domains() {
    for kind in [
        MetricInputKind::Returns,
        MetricInputKind::RawPnl,
        MetricInputKind::Trades,
    ] {
        let mut metric = BreakevenRate::new(NanPolicy::Omit)
            .and_then(|mut state| {
                match kind {
                    MetricInputKind::Returns => {
                        state.from_returns(&[])?;
                    }
                    MetricInputKind::RawPnl => {
                        state.from_pnl(&[])?;
                    }
                    MetricInputKind::Trades => {
                        state.from_trades(&[])?;
                    }
                    _ => {
                        state.append(0.0)?;
                    }
                }
                Ok(state)
            })
            .unwrap();
        assert_eq!(metric.extend(&[1.0, 0.0, -1.0, -0.0]).unwrap(), Some(0.5));
    }
}
#[test]
fn omission_and_lifecycle_are_invariant() {
    let mut metric = BreakevenRate::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    metric.append(0.0).unwrap();
    metric.extend(&[f64::NAN, 2.0, 0.0]).unwrap();
    assert_eq!(metric.value(), Some(2.0 / 3.0));
    assert_eq!(metric.len(), 3);
    metric.reset();
    assert_eq!(metric.extend(&[0.0, 2.0, 0.0]).unwrap(), Some(2.0 / 3.0));
}
#[test]
fn validates_domain_and_nan_policy() {
    assert!(BreakevenRate::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    let mut metric = BreakevenRate::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert!(metric.append(f64::NAN).is_err());
    assert_eq!(metric.len(), 0);
}
