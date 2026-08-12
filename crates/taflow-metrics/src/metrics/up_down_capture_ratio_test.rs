use super::up_down_capture_ratio::UpDownCaptureRatio;
use crate::{MetricInputKind, NanPolicy};

fn annualized(returns: &[f64], periods_per_year: f64) -> f64 {
    (returns.iter().map(|value| value.ln_1p()).sum::<f64>() * periods_per_year
        / returns.len() as f64)
        .exp_m1()
}

fn expected(primary: &[f64], benchmark: &[f64], periods_per_year: f64) -> f64 {
    let up_primary: Vec<_> = primary
        .iter()
        .zip(benchmark)
        .filter_map(|(&value, &reference)| (reference > 0.0).then_some(value))
        .collect();
    let up_benchmark: Vec<_> = benchmark
        .iter()
        .copied()
        .filter(|value| *value > 0.0)
        .collect();
    let down_primary: Vec<_> = primary
        .iter()
        .zip(benchmark)
        .filter_map(|(&value, &reference)| (reference < 0.0).then_some(value))
        .collect();
    let down_benchmark: Vec<_> = benchmark
        .iter()
        .copied()
        .filter(|value| *value < 0.0)
        .collect();
    let up =
        annualized(&up_primary, periods_per_year) / annualized(&up_benchmark, periods_per_year);
    let down =
        annualized(&down_primary, periods_per_year) / annualized(&down_benchmark, periods_per_year);
    up / down
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn matches_filtered_capture_quotient_and_preserves_lifecycle() {
    let primary = [0.03, -0.01, 0.02, -0.04, 0.05];
    let benchmark = [0.01, -0.02, 0.0, -0.015, 0.02];
    let expected = expected(&primary, &benchmark, 252.0);
    let mut state = UpDownCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(primary[0], benchmark[0]).unwrap(), None);
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
fn pairwise_omission_counts_all_usable_pairs_and_requires_both_sides() {
    let mut state = UpDownCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    state
        .extend(
            &[0.10, f64::NAN, 0.05, -0.02, 0.08],
            &[0.02, 0.04, f64::NAN, -0.01, 0.0],
        )
        .unwrap();
    assert_eq!(state.len(), 3);
    assert_close(
        state.value().unwrap(),
        expected(&[0.10, -0.02], &[0.02, -0.01], 12.0),
    );

    let mut only_up = UpDownCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    only_up.extend(&[0.01, 0.02], &[0.01, 0.02]).unwrap();
    assert_eq!(only_up.value(), None);

    let mut zero_down_capture = UpDownCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    zero_down_capture
        .extend(&[0.02, 0.0], &[0.01, -0.01])
        .unwrap();
    assert_eq!(zero_down_capture.value(), None);
}

#[test]
fn input_modes_are_equivalent() {
    let primary_returns = [0.10, -0.20, 0.05];
    let benchmark_returns = [0.02, -0.10, 0.01];
    let expected = expected(&primary_returns, &benchmark_returns, 12.0);

    let cases = [
        (
            MetricInputKind::Returns,
            MetricInputKind::Returns,
            primary_returns.to_vec(),
            benchmark_returns.to_vec(),
        ),
        (
            MetricInputKind::LogReturns,
            MetricInputKind::LogReturns,
            primary_returns.map(f64::ln_1p).to_vec(),
            benchmark_returns.map(f64::ln_1p).to_vec(),
        ),
    ];
    for (primary_kind, benchmark_kind, primary, benchmark) in cases {
        let mut state = UpDownCaptureRatio::new(12.0, NanPolicy::Omit).unwrap();
        match (primary_kind, benchmark_kind) {
            (MetricInputKind::Returns, MetricInputKind::Returns) => {
                state.from_returns(&[], &[]).unwrap();
            }
            (MetricInputKind::LogReturns, MetricInputKind::LogReturns) => {
                state.from_log_returns(&[], &[]).unwrap();
            }
            _ => unreachable!(),
        }
        assert_close(
            state.extend(&primary, &benchmark).unwrap().unwrap(),
            expected,
        );
    }

    let mut equity = UpDownCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_equity(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        equity
            .extend(&[100.0, 110.0, 88.0, 92.4], &[200.0, 204.0, 183.6, 185.436])
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut pnl = UpDownCaptureRatio::new(12.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_pnl(&[], &[], 100.0, 200.0)?;
            Ok(state)
        })
        .unwrap();
    assert_close(
        pnl.extend(&[10.0, -22.0, 4.4], &[4.0, -20.4, 1.836])
            .unwrap()
            .unwrap(),
        expected,
    );
}

#[test]
fn rejects_invalid_domains_and_misalignment_transactionally() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(UpDownCaptureRatio::new(invalid, NanPolicy::Omit)
            .and_then(|mut state| {
                state.from_returns(&[], &[])?;
                Ok(state)
            })
            .is_err());
    }
    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        assert!(UpDownCaptureRatio::new(252.0, NanPolicy::Omit)
            .and_then(|mut state| {
                state.append(0.0, 0.0)?;
                Ok(state)
            })
            .is_err());
    }

    let mut state = UpDownCaptureRatio::new(252.0, NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[], &[])?;
            Ok(state)
        })
        .unwrap();
    assert!(state.extend(&[0.01, -0.02], &[0.01]).is_err());
    assert_eq!(state.len(), 0);
    assert_eq!(state.value(), None);
}
