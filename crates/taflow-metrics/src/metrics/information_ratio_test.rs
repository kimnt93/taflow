use super::information_ratio::InformationRatio;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

fn expected(primary: &[f64], benchmark: &[f64], scale: f64) -> f64 {
    let active: Vec<f64> = primary
        .iter()
        .zip(benchmark)
        .map(|(primary, benchmark)| primary - benchmark)
        .collect();
    let mean = active.iter().sum::<f64>() / active.len() as f64;
    let variance = active
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (active.len() - 1) as f64;
    mean / variance.sqrt() * scale
}

#[test]
fn computes_annualized_mean_active_return_over_sample_deviation() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, 0.025, 0.01];
    let expected = expected(&primary, &benchmark, 252.0_f64.sqrt());
    let mut state = InformationRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        true,
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
fn supports_unannualized_output_and_pairwise_missing_omission() {
    let mut state = InformationRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        12.0,
        false,
        NanPolicy::Omit,
    )
    .unwrap();
    state
        .extend(
            &[0.03, f64::NAN, 0.01, -0.02],
            &[0.01, 0.02, f64::NAN, -0.01],
        )
        .unwrap();

    assert_close(
        state.value().unwrap(),
        expected(&[0.03, -0.02], &[0.01, -0.01], 1.0),
    );
    assert_eq!(state.len(), 2);
}

#[test]
fn input_modes_produce_equivalent_information_ratio() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = InformationRatio::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            12.0,
            true,
            NanPolicy::Omit,
        )
        .unwrap();
        state
            .extend(&primary_returns, &benchmark_returns)
            .unwrap()
            .unwrap()
    };

    let mut equity = InformationRatio::new(
        MetricInputKind::Equity,
        MetricInputKind::Equity,
        12.0,
        true,
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

    let mut pnl = InformationRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        MetricInputKind::PeriodPnl {
            initial_equity: 200.0,
        },
        12.0,
        true,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = InformationRatio::new(
        MetricInputKind::LogReturns,
        MetricInputKind::LogReturns,
        12.0,
        true,
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
fn handles_minimum_constant_and_rejects_misalignment_without_mutation() {
    let mut state = InformationRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        true,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_eq!(state.append(0.01, 0.0).unwrap(), None);
    assert!(state.extend(&[0.02, 0.03], &[0.01]).is_err());
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), None);
    state.append(0.02, 0.01).unwrap();
    assert_eq!(state.value(), None);
}

#[test]
fn rejects_invalid_configuration_and_domains() {
    for periods_per_year in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(InformationRatio::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            periods_per_year,
            true,
            NanPolicy::Omit,
        )
        .is_err());
    }
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        assert!(InformationRatio::new(kind, kind, 252.0, true, NanPolicy::Omit).is_err());
    }
}
