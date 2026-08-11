use super::LongestLosingStreak;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn counts_strict_negative_runs() {
    let mut m = LongestLosingStreak::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
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
        let mut m = LongestLosingStreak::new(kind, NanPolicy::Omit).unwrap();
        assert_eq!(m.extend(&[-0.1, f64::NAN, -0.2, 0.0]).unwrap(), Some(2));
        m.reset();
        assert_eq!(m.extend(&[-0.1, -0.2, 0.0]).unwrap(), Some(2));
    }
}
#[test]
fn edges_and_validation() {
    let mut m = LongestLosingStreak::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(m.value(), None);
    assert_eq!(m.extend(&[0.0, 0.2]).unwrap(), Some(0));
    assert!(LongestLosingStreak::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
}
