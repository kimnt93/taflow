use super::down_market_capture_ratio::DownMarketCaptureRatio;
use crate::{MetricInputKind, NanPolicy};

fn annualized(returns: &[f64], periods_per_year: f64) -> f64 {
    (returns.iter().map(|value| value.ln_1p()).sum::<f64>() * periods_per_year
        / returns.len() as f64)
        .exp_m1()
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn filters_negative_benchmark_periods_and_preserves_lifecycle() {
    let primary = [0.03, -0.01, 0.02, 0.04];
    let benchmark = [0.01, -0.02, -0.025, 0.01];
    let expected = annualized(&[-0.01, 0.02], 12.0) / annualized(&[-0.02, -0.025], 12.0);
    let mut state = DownMarketCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(primary[0], benchmark[0]).unwrap(), None);
    state.extend(&primary[1..3], &benchmark[1..3]).unwrap();
    assert_close(
        state.append(primary[3], benchmark[3]).unwrap().unwrap(),
        expected,
    );
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), 4);
    assert_eq!(state.eligible_count(), 2);
    assert_eq!(state.periods_per_year(), 12.0);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(
        state.extend(&primary, &benchmark).unwrap().unwrap(),
        expected,
    );
}

#[test]
fn omits_missing_pairs_and_requires_an_eligible_observation() {
    let mut state = DownMarketCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    state
        .extend(
            &[0.10, f64::NAN, 0.05, -0.02],
            &[0.02, -0.04, f64::NAN, -0.01],
        )
        .unwrap();
    assert_close(
        state.value().unwrap(),
        annualized(&[-0.02], 252.0) / annualized(&[-0.01], 252.0),
    );
    assert_eq!(state.len(), 2);
    assert_eq!(state.eligible_count(), 1);

    let mut only_up = DownMarketCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    only_up.extend(&[0.01, -0.02], &[0.02, 0.0]).unwrap();
    assert_eq!(only_up.value(), None);
}

#[test]
fn input_modes_produce_equivalent_results() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, -0.01];
    let mut returns = DownMarketCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    let expected = returns
        .extend(&primary_returns, &benchmark_returns)
        .unwrap()
        .unwrap();

    let mut equity = DownMarketCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4], &[200.0, 204.0, 183.6, 181.764])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut pnl = DownMarketCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], &[], 100.0, 200.0)?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, -1.836])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut logarithmic = DownMarketCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_log_returns(&[], &[])?;
            Ok(state)
        })
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
fn rejects_invalid_configuration_misalignment_and_domains_transactionally() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(DownMarketCaptureRatio::new(invalid, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[], &[])?;
                Ok(state)
            })
            .is_err());
    }
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        assert!(DownMarketCaptureRatio::new(252.0, NanPolicy::Omit)
            .and_then(|mut state| {
                state.append(0.0, 0.0)?;
                Ok(state)
            })
            .is_err());
    }

    let mut state = DownMarketCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.extend(&[0.01, 0.02], &[-0.01]).is_err());
    assert_eq!(state.len(), 0);
}
