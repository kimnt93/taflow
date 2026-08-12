use super::NetProfit;
use crate::{MetricInputKind, NanPolicy};
#[test]
fn sums_signed_profit_and_loss() {
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let mut m = NetProfit::new(NanPolicy::Omit).unwrap();
        match kind {
            MetricInputKind::RawPnl => m.from_pnl(&[]).unwrap(),
            MetricInputKind::Trades => m.from_trades(&[]).unwrap(),
            _ => unreachable!(),
        };
        assert_eq!(m.extend(&[10.0, -4.0, 0.0, -1.0]).unwrap(), Some(5.0));
    }
}
#[test]
fn lifecycle_nan_and_edges() {
    let mut m = NetProfit::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_trades(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(m.extend(&[f64::NAN, 0.0]).unwrap(), Some(0.0));
    m.reset();
    assert_eq!(m.value(), None);
    assert_eq!(m.extend(&[2.0, -1.0]).unwrap(), Some(1.0));
}
#[test]
fn rejects_converted_domains() {
    assert!(NetProfit::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
