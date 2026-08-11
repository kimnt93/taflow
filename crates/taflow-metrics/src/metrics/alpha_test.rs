use super::alpha::Alpha;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

fn expected_alpha(
    primary: &[f64],
    benchmark: &[f64],
    periods_per_year: f64,
    annual_risk_free_rate: f64,
) -> f64 {
    let count = primary.len() as f64;
    let primary_mean = primary.iter().sum::<f64>() / count;
    let benchmark_mean = benchmark.iter().sum::<f64>() / count;
    let covariance = primary
        .iter()
        .zip(benchmark)
        .map(|(primary, benchmark)| (primary - primary_mean) * (benchmark - benchmark_mean))
        .sum::<f64>()
        / count;
    let benchmark_variance = benchmark
        .iter()
        .map(|benchmark| (benchmark - benchmark_mean).powi(2))
        .sum::<f64>()
        / count;
    let beta = covariance / benchmark_variance;
    let period_risk_free_rate = (annual_risk_free_rate.ln_1p() / periods_per_year).exp_m1();
    let intercept =
        (primary_mean - period_risk_free_rate) - beta * (benchmark_mean - period_risk_free_rate);
    (1.0 + intercept).powf(periods_per_year) - 1.0
}

#[test]
fn computes_compounded_annualized_regression_intercept() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, 0.025, 0.01];
    let expected = expected_alpha(&primary, &benchmark, 12.0, 0.04);
    let mut state = Alpha::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        12.0,
        0.04,
        NanPolicy::Omit,
    )
    .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(primary[0], benchmark[0]).unwrap(), None);
    state.extend(&primary[1..3], &benchmark[1..3]).unwrap();
    assert_close(
        state.append(primary[3], benchmark[3]).unwrap().unwrap(),
        expected,
    );
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), primary.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(
        state.extend(&primary, &benchmark).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn omits_missing_values_pairwise_and_handles_undefined_cases() {
    let mut state = Alpha::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        1.0,
        0.0,
        NanPolicy::Omit,
    )
    .unwrap();
    state
        .extend(
            &[0.01, f64::NAN, 0.05, -0.02],
            &[0.00, 0.02, f64::NAN, -0.01],
        )
        .unwrap();
    assert_close(state.value().unwrap(), 0.01);
    assert_eq!(state.len(), 2);

    let mut constant_benchmark = Alpha::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        0.0,
        NanPolicy::Omit,
    )
    .unwrap();
    constant_benchmark
        .extend(&[0.01, 0.02, 0.03], &[0.01, 0.01, 0.01])
        .unwrap();
    assert_eq!(constant_benchmark.value(), None);
}

#[test]
fn input_modes_produce_equivalent_alpha() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = Alpha::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            12.0,
            0.04,
            NanPolicy::Omit,
        )
        .unwrap();
        state
            .extend(&primary_returns, &benchmark_returns)
            .unwrap()
            .unwrap()
    };

    let mut equity = Alpha::new(
        MetricInputKind::Equity,
        MetricInputKind::Equity,
        12.0,
        0.04,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4], &[200.0, 204.0, 183.6, 185.436])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut pnl = Alpha::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        MetricInputKind::PeriodPnl {
            initial_equity: 200.0,
        },
        12.0,
        0.04,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = Alpha::new(
        MetricInputKind::LogReturns,
        MetricInputKind::LogReturns,
        12.0,
        0.04,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        logarithmic
            .extend(
                &primary_returns.map(f64::ln_1p),
                &benchmark_returns.map(f64::ln_1p),
            )
            .unwrap()
            .unwrap(),
        expected,
    );
}

#[test]
fn rejects_misalignment_invalid_configuration_domains_and_nan() {
    let mut state = Alpha::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        0.0,
        NanPolicy::Omit,
    )
    .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);

    for periods in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(Alpha::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            periods,
            0.0,
            NanPolicy::Omit,
        )
        .is_err());
    }
    for rate in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(Alpha::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            252.0,
            rate,
            NanPolicy::Omit,
        )
        .is_err());
    }
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        assert!(Alpha::new(kind, kind, 252.0, 0.0, NanPolicy::Omit).is_err());
    }

    let mut raises = Alpha::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        0.0,
        NanPolicy::Raise,
    )
    .unwrap();
    assert!(raises.append(f64::NAN, 0.01).is_err());
    assert_eq!(raises.len(), 0);
}
