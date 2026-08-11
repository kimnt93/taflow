use super::maximum_drawdown::MaximumDrawdown;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn uses_phantom_starting_wealth_and_returns_signed_decline() {
    let mut metric = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    metric.extend(&[-0.2, 0.5, -0.5]).unwrap();

    // Wealth is 1 -> 0.8 -> 1.2 -> 0.6, so the deepest decline is -50%.
    assert_eq!(metric.value(), Some(-0.5));
    assert_eq!(metric.compute(), Some(-0.5));
    assert_eq!(metric.len(), 3);
}

#[test]
fn positive_first_return_does_not_create_a_drawdown() {
    let mut metric = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(metric.append(0.25).unwrap(), Some(0.0));
    assert!((metric.append(-0.1).unwrap().unwrap() + 0.1).abs() < 1e-15);
}

#[test]
fn all_semantic_input_modes_have_the_same_result() {
    let returns = [0.1, -0.2, 0.05];
    let expected = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit)
        .and_then(|mut state| {
            state.extend(&returns)?;
            Ok(state.compute())
        })
        .unwrap();

    let mut log_returns =
        MaximumDrawdown::new(MetricInputKind::LogReturns, NanPolicy::Omit).unwrap();
    log_returns.extend(&returns.map(f64::ln_1p)).unwrap();

    let mut equity = MaximumDrawdown::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap();

    let mut pnl = MaximumDrawdown::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    pnl.extend(&[10.0, -22.0, 4.4]).unwrap();

    for actual in [log_returns.compute(), equity.compute(), pnl.compute()] {
        assert!((actual.unwrap() - expected.unwrap()).abs() < 1e-15);
    }
}

#[test]
fn reset_replay_and_chunking_preserve_lifecycle() {
    let returns = [0.1, -0.2, 0.05, -0.25, 0.1];
    let mut batch = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    batch.extend(&returns).unwrap();

    let mut chunked = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    chunked.extend(&returns[..2]).unwrap();
    for value in &returns[2..] {
        chunked.append(*value).unwrap();
    }
    assert_eq!(chunked.compute(), batch.compute());

    chunked.reset();
    assert!(chunked.is_empty());
    assert_eq!(chunked.value(), None);
    chunked.extend(&returns).unwrap();
    assert_eq!(chunked.compute(), batch.compute());
}

#[test]
fn omitted_nan_does_not_advance_usable_length() {
    let mut metric = MaximumDrawdown::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    metric.extend(&[0.1, f64::NAN, -0.25]).unwrap();
    assert_eq!(metric.len(), 2);
    assert_eq!(metric.value(), Some(-0.25));
}

#[test]
fn rejects_non_return_semantic_domains() {
    assert!(MaximumDrawdown::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    assert!(MaximumDrawdown::new(MetricInputKind::Trades, NanPolicy::Omit).is_err());
}
