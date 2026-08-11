use super::annualized_return::AnnualizedReturn;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-13, "{actual} != {expected}");
}

fn expected_annualized(returns: &[f64], periods_per_year: f64) -> f64 {
    let product = returns.iter().map(|value| 1.0 + value).product::<f64>();
    product.powf(periods_per_year / returns.len() as f64) - 1.0
}

#[test]
fn computes_geometric_annualized_return_and_preserves_lifecycle() {
    let returns = [0.10, -0.20, 0.05, 0.03];
    let expected = expected_annualized(&returns, 12.0);
    let mut state = AnnualizedReturn::new(MetricInputKind::Returns, 12.0, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.compute(), None);
    state.extend(&returns[..2]).unwrap();
    state.append(returns[2]).unwrap();
    assert_close(state.append(returns[3]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), 4);
    assert_eq!(state.periods_per_year(), 12.0);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn all_return_input_modes_are_equivalent() {
    let returns = [0.10, -0.20, 0.05];
    let expected = expected_annualized(&returns, 252.0);

    let mut equity =
        AnnualizedReturn::new(MetricInputKind::Equity, 252.0, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = AnnualizedReturn::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        252.0,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        AnnualizedReturn::new(MetricInputKind::LogReturns, 252.0, NanPolicy::Omit).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn omits_nan_and_represents_total_loss() {
    let mut state =
        AnnualizedReturn::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, 0.25, -1.0]).unwrap();
    assert_eq!(state.value(), Some(-1.0));
    assert_eq!(state.len(), 2);
}

#[test]
fn validates_configuration_and_semantic_domain() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(AnnualizedReturn::new(MetricInputKind::Returns, invalid, NanPolicy::Omit).is_err());
    }
    assert!(AnnualizedReturn::new(MetricInputKind::RawPnl, 252.0, NanPolicy::Omit).is_err());
    assert!(AnnualizedReturn::new(MetricInputKind::Trades, 252.0, NanPolicy::Omit).is_err());
}
