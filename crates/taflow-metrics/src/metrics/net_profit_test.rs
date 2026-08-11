use super::NetProfit;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn sums_signed_profit_and_loss() {
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let mut m = NetProfit::new(kind, NanPolicy::Omit).unwrap();
        assert_eq!(m.extend(&[10.0, -4.0, 0.0, -1.0]).unwrap(), Some(5.0));
    }
}
#[test]
fn lifecycle_nan_and_edges() {
    let mut m = NetProfit::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    assert_eq!(m.extend(&[f64::NAN, 0.0]).unwrap(), Some(0.0));
    m.reset();
    assert_eq!(m.value(), None);
    assert_eq!(m.extend(&[2.0, -1.0]).unwrap(), Some(1.0));
}
#[test]
fn rejects_converted_domains() {
    assert!(NetProfit::new(MetricInputKind::Returns, NanPolicy::Omit).is_err());
}
