use super::sharpe_ratio::SharpeRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-13, "{actual} != {expected}");
}

#[test]
fn computes_sample_sharpe_and_preserves_lifecycle() {
    let returns = [0.01, -0.02, 0.03, 0.00];
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (returns.len() - 1) as f64;
    let expected = mean / variance.sqrt() * 252.0_f64.sqrt();
    let mut state =
        SharpeRatio::new(MetricInputKind::Returns, 252.0, 0.0, NanPolicy::Omit).unwrap();

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
fn converts_annual_effective_risk_free_rate_per_period() {
    let returns = [0.02, -0.01, 0.03, 0.01];
    let periods_per_year = 12.0;
    let annual_risk_free_rate = 0.12682503013196977;
    let period_rate = (1.0_f64 + annual_risk_free_rate).powf(1.0 / periods_per_year) - 1.0;
    let excess = returns.map(|value| value - period_rate);
    let mean = excess.iter().sum::<f64>() / excess.len() as f64;
    let variance = excess
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (excess.len() - 1) as f64;
    let expected = mean / variance.sqrt() * periods_per_year.sqrt();

    let mut state = SharpeRatio::new(
        MetricInputKind::Returns,
        periods_per_year,
        annual_risk_free_rate,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn input_modes_produce_equivalent_sharpe_ratio() {
    let returns = [0.10, -0.20, 0.05];
    let expected = {
        let mut state =
            SharpeRatio::new(MetricInputKind::Returns, 12.0, 0.04, NanPolicy::Omit).unwrap();
        state.extend(&returns).unwrap().unwrap()
    };

    let mut equity =
        SharpeRatio::new(MetricInputKind::Equity, 12.0, 0.04, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = SharpeRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        12.0,
        0.04,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        SharpeRatio::new(MetricInputKind::LogReturns, 12.0, 0.04, NanPolicy::Omit).unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn handles_missing_minimum_and_zero_deviation_samples() {
    let mut state =
        SharpeRatio::new(MetricInputKind::Returns, 252.0, 0.0, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, 0.25]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), None);
    state.append(0.25).unwrap();
    assert_eq!(state.value(), None);
}

#[test]
fn rejects_invalid_annual_settings() {
    for periods in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(SharpeRatio::new(MetricInputKind::Returns, periods, 0.0, NanPolicy::Omit).is_err());
    }
    for rate in [-1.0, -2.0, f64::NAN, f64::INFINITY] {
        assert!(SharpeRatio::new(MetricInputKind::Returns, 252.0, rate, NanPolicy::Omit).is_err());
    }
    assert!(SharpeRatio::new(MetricInputKind::RawPnl, 252.0, 0.0, NanPolicy::Omit).is_err());
    assert!(SharpeRatio::new(MetricInputKind::Trades, 252.0, 0.0, NanPolicy::Omit).is_err());
}
