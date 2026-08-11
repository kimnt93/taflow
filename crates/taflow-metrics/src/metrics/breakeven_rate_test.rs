use super::BreakevenRate;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn counts_exact_zero_across_supported_domains() {
    for kind in [
        MetricInputKind::Returns,
        MetricInputKind::RawPnl,
        MetricInputKind::Trades,
    ] {
        let mut metric = BreakevenRate::new(kind, NanPolicy::Omit).unwrap();
        assert_eq!(metric.extend(&[1.0, 0.0, -1.0, -0.0]).unwrap(), Some(0.5));
    }
}
#[test]
fn omission_and_lifecycle_are_invariant() {
    let mut metric = BreakevenRate::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    metric.append(0.0).unwrap();
    metric.extend(&[f64::NAN, 2.0, 0.0]).unwrap();
    assert_eq!(metric.value(), Some(2.0 / 3.0));
    assert_eq!(metric.len(), 3);
    metric.reset();
    assert_eq!(metric.extend(&[0.0, 2.0, 0.0]).unwrap(), Some(2.0 / 3.0));
}
#[test]
fn validates_domain_and_nan_policy() {
    assert!(BreakevenRate::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    let mut metric = BreakevenRate::new(MetricInputKind::Returns, NanPolicy::Raise).unwrap();
    assert!(metric.append(f64::NAN).is_err());
    assert_eq!(metric.len(), 0);
}
