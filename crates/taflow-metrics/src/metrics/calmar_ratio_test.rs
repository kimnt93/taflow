use super::calmar_ratio::CalmarRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-12 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn computes_geometric_annual_return_over_absolute_drawdown() {
    let returns = [0.10, -0.20, 0.05];
    let mut metric = CalmarRatio::new(MetricInputKind::Returns, 12.0, NanPolicy::Omit).unwrap();
    metric.extend(&returns).unwrap();

    let annualized_return = ((1.10_f64 * 0.80 * 1.05).ln() * 4.0).exp_m1();
    let maximum_drawdown: f64 = -0.20;
    assert_close(
        metric.value().unwrap(),
        annualized_return / maximum_drawdown.abs(),
    );
    assert_eq!(metric.compute(), metric.value());
    assert_eq!(metric.len(), returns.len());
    assert_eq!(metric.periods_per_year(), 12.0);
}

#[test]
fn semantic_input_modes_and_streaming_are_invariant() {
    let returns: [f64; 5] = [0.10, -0.20, 0.05, -0.25, 0.10];
    let equity = [100.0, 110.0, 88.0, 92.4, 69.3, 76.23];
    let pnl = [10.0, -22.0, 4.4, -23.1, 6.93];
    let log_returns: Vec<f64> = returns.iter().map(|value| value.ln_1p()).collect();

    let mut expected = CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    expected.extend(&returns).unwrap();
    let expected_value = expected.value().unwrap();

    let mut from_equity =
        CalmarRatio::new(MetricInputKind::Equity, 252.0, NanPolicy::Omit).unwrap();
    from_equity.extend(&equity).unwrap();
    assert_close(from_equity.value().unwrap(), expected_value);

    let mut from_pnl = CalmarRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        252.0,
        NanPolicy::Omit,
    )
    .unwrap();
    from_pnl.extend(&pnl).unwrap();
    assert_close(from_pnl.value().unwrap(), expected_value);

    let mut from_log =
        CalmarRatio::new(MetricInputKind::LogReturns, 252.0, NanPolicy::Omit).unwrap();
    from_log.extend(&log_returns).unwrap();
    assert_close(from_log.value().unwrap(), expected_value);

    let mut streamed = CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    streamed.append(returns[0]).unwrap();
    streamed.extend(&returns[1..3]).unwrap();
    streamed.extend(&returns[3..]).unwrap();
    assert_close(streamed.value().unwrap(), expected_value);

    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&returns).unwrap();
    assert_close(streamed.value().unwrap(), expected_value);
}

#[test]
fn undefined_and_missing_value_contract_is_explicit() {
    let mut empty = CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    assert_eq!(empty.value(), None);
    empty.extend(&[0.10, 0.0, 0.20]).unwrap();
    assert_eq!(empty.value(), None);

    let mut omitted = CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    omitted.extend(&[f64::NAN, -0.10, f64::NAN]).unwrap();
    assert_eq!(omitted.len(), 1);
    assert!(omitted.value().is_some());

    let mut total_loss =
        CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Omit).unwrap();
    total_loss.append(-1.0).unwrap();
    assert_eq!(total_loss.value(), Some(-1.0));
}

#[test]
fn rejects_invalid_configuration_and_observations() {
    for periods_per_year in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            CalmarRatio::new(MetricInputKind::Returns, periods_per_year, NanPolicy::Omit).is_err()
        );
    }
    assert!(CalmarRatio::new(MetricInputKind::RawPnl, 252.0, NanPolicy::Omit).is_err());
    assert!(CalmarRatio::new(MetricInputKind::Trades, 252.0, NanPolicy::Omit).is_err());

    let mut state = CalmarRatio::new(MetricInputKind::Returns, 252.0, NanPolicy::Raise).unwrap();
    assert!(state.append(f64::NAN).is_err());
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
}
