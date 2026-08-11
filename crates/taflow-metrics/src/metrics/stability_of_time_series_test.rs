use super::StabilityOfTimeSeries;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn computes_squared_correlation_of_cumulative_log_returns() {
    let returns = [0.01, -0.02, 0.03, 0.015];
    let mut metric = StabilityOfTimeSeries::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    metric.extend(&returns).unwrap();
    let cumulative: Vec<f64> = returns
        .iter()
        .scan(0.0, |sum, value| {
            *sum += value.ln_1p();
            Some(*sum)
        })
        .collect();
    let x_mean = 1.5;
    let y_mean = cumulative.iter().sum::<f64>() / 4.0;
    let covariance = (0..4)
        .map(|index| (index as f64 - x_mean) * (cumulative[index] - y_mean))
        .sum::<f64>();
    let denominator = ((0..4)
        .map(|index| (index as f64 - x_mean).powi(2))
        .sum::<f64>()
        * cumulative
            .iter()
            .map(|value| (value - y_mean).powi(2))
            .sum::<f64>())
    .sqrt();
    assert!((metric.value().unwrap() - (covariance / denominator).powi(2)).abs() < 1e-14);
}

#[test]
fn lifecycle_nan_and_warmup_are_invariant() {
    let mut metric = StabilityOfTimeSeries::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(metric.append(0.01).unwrap(), None);
    metric.append(f64::NAN).unwrap();
    let expected = metric.append(0.02).unwrap();
    assert_eq!(metric.len(), 2);
    metric.reset();
    assert_eq!(metric.extend(&[0.01, 0.02]).unwrap(), expected);
}

#[test]
fn semantic_input_modes_are_equivalent() {
    let returns: [f64; 3] = [0.01, -0.02, 0.03];
    let log_returns: Vec<_> = returns.iter().map(|value| value.ln_1p()).collect();
    let equity = [100.0, 101.0, 98.98, 101.9494];
    let pnl = [1.0, -2.02, 2.9694];
    let mut from_returns =
        StabilityOfTimeSeries::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    let mut from_logs =
        StabilityOfTimeSeries::new(MetricInputKind::LogReturns, NanPolicy::Omit).unwrap();
    let mut from_equity =
        StabilityOfTimeSeries::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    let mut from_pnl = StabilityOfTimeSeries::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    let expected = from_returns.extend(&returns).unwrap().unwrap();
    assert!((from_logs.extend(&log_returns).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((from_equity.extend(&equity).unwrap().unwrap() - expected).abs() < 1e-14);
    assert!((from_pnl.extend(&pnl).unwrap().unwrap() - expected).abs() < 1e-14);
}

#[test]
fn rejects_non_return_domains_and_total_loss_is_undefined() {
    assert!(StabilityOfTimeSeries::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    let mut metric = StabilityOfTimeSeries::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert_eq!(metric.extend(&[0.1, -1.0, 0.2]).unwrap(), None);
}
