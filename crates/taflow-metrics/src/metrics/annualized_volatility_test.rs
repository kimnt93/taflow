use super::annualized_volatility::AnnualizedVolatility;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn computes_sample_volatility_and_preserves_lifecycle() {
    let returns = [0.01, -0.02, 0.03, 0.00];
    let expected = 0.02081665999466133 * 252.0_f64.sqrt();
    let mut state =
        AnnualizedVolatility::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(returns[0]).unwrap(), None);
    state.extend(&returns[1..3]).unwrap();
    assert_close(state.append(returns[3]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), returns.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn input_modes_produce_equivalent_volatility() {
    let returns = [0.10, -0.20, 0.05];
    let expected = {
        let mut state =
            AnnualizedVolatility::new(MetricInputKind::Returns, 12.0, NanPolicy::Omit).unwrap();
        state.extend(&returns).unwrap().unwrap()
    };

    let mut equity =
        AnnualizedVolatility::new(MetricInputKind::Equity, 12.0, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = AnnualizedVolatility::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        12.0,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        AnnualizedVolatility::new(MetricInputKind::LogReturns, 12.0, NanPolicy::Omit).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn handles_missing_constant_and_minimum_samples() {
    let mut state =
        AnnualizedVolatility::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, 0.25]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), None);
    state.append(0.25).unwrap();
    assert_eq!(state.value(), Some(0.0));
}

#[test]
fn rejects_invalid_periods_per_year() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            AnnualizedVolatility::new(MetricInputKind::Returns, value, NanPolicy::Omit).is_err()
        );
    }
}

#[test]
fn rejects_non_return_semantic_domains() {
    assert!(AnnualizedVolatility::new(MetricInputKind::RawPnl, 252.0, NanPolicy::Omit).is_err());
    assert!(AnnualizedVolatility::new(MetricInputKind::Trades, 252.0, NanPolicy::Omit).is_err());
}
