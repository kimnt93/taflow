use super::LongestLosingStreak;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn counts_strict_negative_runs() {
    let mut m = LongestLosingStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(
        m.extend(&[-1.0, -2.0, 0.0, -1.0, -1.0, -1.0, 2.0]).unwrap(),
        Some(3)
    );
}
#[test]
fn lifecycle_and_domains() {
    for kind in [
        MetricInputKind::Returns,
        MetricInputKind::RawPnl,
        MetricInputKind::Trades,
    ] {
        let mut m = LongestLosingStreak::new(NanPolicy::Omit).unwrap();
        match kind {
            MetricInputKind::Returns => m.from_returns(&[]).unwrap(),
            MetricInputKind::RawPnl => m.from_pnl(&[]).unwrap(),
            MetricInputKind::Trades => m.from_trades(&[]).unwrap(),
            _ => unreachable!(),
        };
        assert_eq!(m.extend(&[-0.1, f64::NAN, -0.2, 0.0]).unwrap(), Some(2));
        m.reset();
        assert_eq!(m.extend(&[-0.1, -0.2, 0.0]).unwrap(), Some(2));
    }
}
#[test]
fn edges_and_validation() {
    let mut m = LongestLosingStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(m.value(), None);
    assert_eq!(m.extend(&[0.0, 0.2]).unwrap(), Some(0));
    assert!(LongestLosingStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
