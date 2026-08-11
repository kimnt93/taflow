use super::coefficient_of_determination::CoefficientOfDetermination;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn computes_squared_pearson_correlation_and_lifecycle() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, 0.025, 0.01];
    let expected = {
        let primary_mean = primary.iter().sum::<f64>() / primary.len() as f64;
        let benchmark_mean = benchmark.iter().sum::<f64>() / benchmark.len() as f64;
        let primary_sum = primary
            .iter()
            .map(|value| (value - primary_mean).powi(2))
            .sum::<f64>();
        let benchmark_sum = benchmark
            .iter()
            .map(|value| (value - benchmark_mean).powi(2))
            .sum::<f64>();
        let cross_sum = primary
            .iter()
            .zip(benchmark)
            .map(|(primary, benchmark)| (primary - primary_mean) * (benchmark - benchmark_mean))
            .sum::<f64>();
        cross_sum.powi(2) / (primary_sum * benchmark_sum)
    };
    let mut state = CoefficientOfDetermination::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
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
    let mut state = CoefficientOfDetermination::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        NanPolicy::Omit,
    )
    .unwrap();
    state
        .extend(
            &[0.01, f64::NAN, 0.05, -0.02],
            &[0.00, 0.02, f64::NAN, -0.01],
        )
        .unwrap();

    assert_close(state.value().unwrap(), 1.0);
    assert_eq!(state.len(), 2);

    for (primary, benchmark) in [
        (&[0.25][..], &[0.10][..]),
        (&[0.01, 0.01][..], &[0.10, 0.20][..]),
        (&[0.01, 0.02][..], &[0.10, 0.10][..]),
    ] {
        let mut undefined = CoefficientOfDetermination::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            NanPolicy::Omit,
        )
        .unwrap();
        undefined.extend(primary, benchmark).unwrap();
        assert_eq!(undefined.value(), None);
    }
}

#[test]
fn input_modes_produce_equivalent_results() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = CoefficientOfDetermination::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            NanPolicy::Omit,
        )
        .unwrap();
        state
            .extend(&primary_returns, &benchmark_returns)
            .unwrap()
            .unwrap()
    };

    let mut equity = CoefficientOfDetermination::new(
        MetricInputKind::Equity,
        MetricInputKind::Equity,
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

    let mut pnl = CoefficientOfDetermination::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        MetricInputKind::PeriodPnl {
            initial_equity: 200.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = CoefficientOfDetermination::new(
        MetricInputKind::LogReturns,
        MetricInputKind::LogReturns,
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
fn rejects_misalignment_invalid_domains_and_nan_when_requested() {
    let mut state = CoefficientOfDetermination::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        NanPolicy::Omit,
    )
    .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);

    assert!(CoefficientOfDetermination::new(
        MetricInputKind::RawPnl,
        MetricInputKind::RawPnl,
        NanPolicy::Omit,
    )
    .is_err());
    assert!(CoefficientOfDetermination::new(
        MetricInputKind::Trades,
        MetricInputKind::Trades,
        NanPolicy::Omit,
    )
    .is_err());

    let mut raises = CoefficientOfDetermination::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        NanPolicy::Raise,
    )
    .unwrap();
    assert!(raises.append(f64::NAN, 0.01).is_err());
    assert_eq!(raises.len(), 0);
}
