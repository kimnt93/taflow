use super::up_market_capture_ratio::UpMarketCaptureRatio;
use crate::{MetricInputKind, NanPolicy};

fn annualized(returns: &[f64], periods_per_year: f64) -> f64 {
    (returns.iter().map(|value| value.ln_1p()).sum::<f64>()
        * (periods_per_year / returns.len() as f64))
        .exp_m1()
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn filters_positive_benchmark_periods_and_preserves_lifecycle() {
    let primary = [0.03, -0.01, 0.02, 0.04, -0.05];
    let benchmark = [0.01, -0.02, 0.0, 0.015, -0.01];
    let expected = annualized(&[0.03, 0.04], 252.0) / annualized(&[0.01, 0.015], 252.0);
    let mut state = UpMarketCaptureRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        NanPolicy::Omit,
    )
    .unwrap();

    assert_eq!(state.value(), None);
    assert_close(state.append(primary[0], benchmark[0]).unwrap().unwrap(), {
        annualized(&primary[..1], 252.0) / annualized(&benchmark[..1], 252.0)
    });
    state.extend(&primary[1..4], &benchmark[1..4]).unwrap();
    assert_close(
        state.append(primary[4], benchmark[4]).unwrap().unwrap(),
        expected,
    );
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), primary.len());
    assert_eq!(state.periods_per_year(), 252.0);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(
        state.extend(&primary, &benchmark).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn omits_missing_pairs_before_filtering_and_requires_an_up_period() {
    let mut state = UpMarketCaptureRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        12.0,
        NanPolicy::Omit,
    )
    .unwrap();
    state
        .extend(
            &[0.10, f64::NAN, 0.05, -0.02, 0.08],
            &[0.02, 0.04, f64::NAN, -0.01, 0.0],
        )
        .unwrap();
    assert_close(
        state.value().unwrap(),
        annualized(&[0.10], 12.0) / annualized(&[0.02], 12.0),
    );
    assert_eq!(state.len(), 3);

    let mut no_up_period = UpMarketCaptureRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        NanPolicy::Omit,
    )
    .unwrap();
    no_up_period.extend(&[0.01, -0.02], &[0.0, -0.01]).unwrap();
    assert_eq!(no_up_period.value(), None);
    assert_eq!(no_up_period.len(), 2);
    assert!(!no_up_period.is_empty());

    let smallest_positive = f64::from_bits(1);
    let mut rounded_zero_benchmark = UpMarketCaptureRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        smallest_positive,
        NanPolicy::Omit,
    )
    .unwrap();
    rounded_zero_benchmark
        .append(0.01, smallest_positive)
        .unwrap();
    assert_eq!(rounded_zero_benchmark.value(), None);
}

#[test]
fn input_modes_produce_equivalent_up_market_capture_ratio() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = {
        let mut state = UpMarketCaptureRatio::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            12.0,
            NanPolicy::Omit,
        )
        .unwrap();
        state
            .extend(&primary_returns, &benchmark_returns)
            .unwrap()
            .unwrap()
    };

    let mut equity = UpMarketCaptureRatio::new(
        MetricInputKind::Equity,
        MetricInputKind::Equity,
        12.0,
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

    let mut pnl = UpMarketCaptureRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        MetricInputKind::PeriodPnl {
            initial_equity: 200.0,
        },
        12.0,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = UpMarketCaptureRatio::new(
        MetricInputKind::LogReturns,
        MetricInputKind::LogReturns,
        12.0,
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
fn rejects_invalid_configuration_domains_and_misalignment_transactionally() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(UpMarketCaptureRatio::new(
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            invalid,
            NanPolicy::Omit,
        )
        .is_err());
    }
    for invalid_domain in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        assert!(
            UpMarketCaptureRatio::new(invalid_domain, invalid_domain, 252.0, NanPolicy::Omit,)
                .is_err()
        );
    }

    let mut state = UpMarketCaptureRatio::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        252.0,
        NanPolicy::Omit,
    )
    .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);
    assert_eq!(state.value(), None);
}
