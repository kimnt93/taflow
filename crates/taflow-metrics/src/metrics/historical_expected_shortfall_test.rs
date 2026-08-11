use super::historical_expected_shortfall::HistoricalExpectedShortfall;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
}

#[test]
fn selects_empyrical_lower_tail_count_and_refreshes_dirty_cache() {
    let mut state =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.25, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(-0.04).unwrap(), Some(-0.04));
    state.extend(&[0.03, -0.02, 0.01]).unwrap();
    assert_eq!(state.value(), Some(-0.04));
    assert_eq!(state.compute(), Some(-0.04));

    state.append(-0.10).unwrap();
    assert_close(state.compute().unwrap(), -0.07);
    assert_eq!(state.len(), 5);

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.compute(), None);
    state.extend(&[-0.04, 0.03, -0.02, 0.01]).unwrap();
    assert_eq!(state.value(), Some(-0.04));
}

#[test]
fn input_modes_produce_equivalent_tail_means() {
    let returns = [0.10, -0.20, 0.05];
    let expected = {
        let mut state =
            HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.4, NanPolicy::Omit)
                .unwrap();
        state.extend(&returns).unwrap().unwrap()
    };

    let mut equity =
        HistoricalExpectedShortfall::new(MetricInputKind::Equity, 0.4, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );
    assert_eq!(equity.len(), 3);

    let mut pnl = HistoricalExpectedShortfall::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        0.4,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let log_returns = returns.map(f64::ln_1p);
    let mut logarithmic =
        HistoricalExpectedShortfall::new(MetricInputKind::LogReturns, 0.4, NanPolicy::Omit)
            .unwrap();
    assert_close(logarithmic.extend(&log_returns).unwrap().unwrap(), expected);
}

#[test]
fn handles_missing_values_and_one_observation_minimum() {
    let mut state =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.05, NanPolicy::Omit).unwrap();
    state.extend(&[f64::NAN, -0.07, f64::NAN]).unwrap();
    assert_eq!(state.len(), 1);
    assert_eq!(state.value(), Some(-0.07));

    let mut raising =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.05, NanPolicy::Raise).unwrap();
    assert!(raising.append(f64::NAN).is_err());
    assert_eq!(raising.len(), 0);
}

#[test]
fn scalar_batch_chunk_and_reset_replay_are_bitwise_invariant() {
    let values = [-0.07, 0.02, -0.03, 0.01, -0.11, 0.04, -0.02];

    let mut scalar =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.4, NanPolicy::Omit).unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }
    let expected = scalar.compute().unwrap().to_bits();

    let mut batch =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.4, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.compute().unwrap().to_bits(), expected);

    let mut chunked =
        HistoricalExpectedShortfall::new(MetricInputKind::Returns, 0.4, NanPolicy::Omit).unwrap();
    chunked.extend(&values[..2]).unwrap();
    chunked.extend(&values[2..5]).unwrap();
    chunked.extend(&values[5..]).unwrap();
    assert_eq!(chunked.compute().unwrap().to_bits(), expected);

    chunked.reset();
    chunked.extend(&values).unwrap();
    assert_eq!(chunked.compute().unwrap().to_bits(), expected);
}

#[test]
fn rejects_invalid_cutoffs_and_non_return_domains() {
    for cutoff in [0.0, 1.0, -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(HistoricalExpectedShortfall::new(
            MetricInputKind::Returns,
            cutoff,
            NanPolicy::Omit,
        )
        .is_err());
    }
    assert!(
        HistoricalExpectedShortfall::new(MetricInputKind::RawPnl, 0.05, NanPolicy::Omit,).is_err()
    );
    assert!(
        HistoricalExpectedShortfall::new(MetricInputKind::Trades, 0.05, NanPolicy::Omit,).is_err()
    );
}
