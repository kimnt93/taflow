use super::ulcer_index::UlcerIndex;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn matches_quantstats_phantom_wealth_and_sample_divisor() {
    let mut metric = UlcerIndex::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    metric.extend(&[0.1, -0.2, 0.25]).unwrap();

    // Wealth is 1 -> 1.1 -> 0.88 -> 1.1 and drawdowns are 0, -0.2, 0.
    // QuantStats 0.0.81 divides the squared sum by returns.len() - 1.
    assert!((metric.value().unwrap() - (0.04_f64 / 2.0).sqrt()).abs() < 1e-15);
    assert_eq!(metric.compute(), metric.value());
    assert_eq!(metric.len(), 3);
}

#[test]
fn requires_two_usable_returns() {
    let mut metric = UlcerIndex::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(metric.value(), None);
    assert_eq!(metric.append(-0.1).unwrap(), None);
    // Both observations remain 10% below the phantom starting peak, and the
    // oracle divisor is n - 1, so sqrt((0.1^2 + 0.1^2) / 1).
    assert!((metric.append(0.0).unwrap().unwrap() - 0.02_f64.sqrt()).abs() < 1e-15);
}

#[test]
fn all_semantic_input_modes_have_the_same_result() {
    let returns = [0.1, -0.2, 0.05, -0.25, 0.1];
    let mut expected = UlcerIndex::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    expected.extend(&returns).unwrap();

    let mut log_returns = UlcerIndex::new(MetricInputKind::LogReturns, NanPolicy::Omit).unwrap();
    log_returns.extend(&returns.map(f64::ln_1p)).unwrap();

    let mut equity = UlcerIndex::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    equity
        .extend(&[100.0, 110.0, 88.0, 92.4, 69.3, 76.23])
        .unwrap();

    let mut pnl = UlcerIndex::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    pnl.extend(&[10.0, -22.0, 4.4, -23.1, 6.93]).unwrap();

    for actual in [log_returns.compute(), equity.compute(), pnl.compute()] {
        assert!((actual.unwrap() - expected.compute().unwrap()).abs() < 1e-14);
    }
}

#[test]
fn reset_replay_chunking_and_nan_omission_are_invariant() {
    let returns = [0.1, f64::NAN, -0.2, 0.05, -0.25, 0.1];
    let mut batch = UlcerIndex::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    batch.extend(&returns).unwrap();

    let mut chunked = UlcerIndex::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    chunked.extend(&returns[..2]).unwrap();
    for &value in &returns[2..] {
        chunked.append(value).unwrap();
    }
    assert_eq!(chunked.compute(), batch.compute());
    assert_eq!(chunked.len(), 5);

    chunked.reset();
    assert!(chunked.is_empty());
    assert_eq!(chunked.value(), None);
    chunked.extend(&returns).unwrap();
    assert_eq!(chunked.compute(), batch.compute());
}

#[test]
fn rejects_non_path_semantic_domains() {
    assert!(UlcerIndex::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    assert!(UlcerIndex::new(MetricInputKind::Trades, NanPolicy::Omit).is_err());
}
