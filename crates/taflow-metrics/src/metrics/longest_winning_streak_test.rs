use super::longest_winning_streak::LongestWinningStreak;
use crate::{MetricInputKind, NanPolicy};

#[test]
fn counts_strictly_positive_runs_and_breaks_on_zero_or_loss() {
    let mut state = LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    state
        .extend(&[0.01, 0.02, 0.0, 0.03, 0.04, 0.05, -0.01, 0.06])
        .unwrap();
    assert_eq!(state.compute(), Some(3.0));
}

#[test]
fn omission_bridges_a_streak_but_raise_rejects_missing() {
    let mut omitted = LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    omitted.extend(&[0.01, f64::NAN, 0.02]).unwrap();
    assert_eq!(omitted.len(), 2);
    assert_eq!(omitted.value(), Some(2.0));

    let mut raised = LongestWinningStreak::new(NanPolicy::Raise)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    raised.append(0.01).unwrap();
    assert!(raised.append(f64::NAN).is_err());
    assert_eq!(raised.value(), Some(1.0));
}

#[test]
fn raw_pnl_and_trade_domains_preserve_sign_observations() {
    for input_kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let mut state = LongestWinningStreak::new(NanPolicy::Omit).unwrap();
        match input_kind {
            MetricInputKind::RawPnl => state.from_pnl(&[]).unwrap(),
            MetricInputKind::Trades => state.from_trades(&[]).unwrap(),
            _ => unreachable!(),
        };
        state
            .extend(&[100.0, 20.0, -40.0, 50.0, 60.0, 70.0, 0.0])
            .unwrap();
        assert_eq!(state.compute(), Some(3.0));
    }
}

#[test]
fn lifecycle_chunking_and_reset_are_invariant() {
    let values = [0.01, 0.02, 0.0, 0.03, f64::NAN, 0.04, 0.05, -0.01];
    let mut batch = LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    batch.extend(&values).unwrap();
    let expected = batch.value();

    let mut streamed = LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    for value in values {
        streamed.append(value).unwrap();
    }
    assert_eq!(streamed.compute(), expected);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_eq!(streamed.compute(), expected);
}

#[test]
fn freezes_empty_and_no_win_results_and_validates_domains() {
    let mut state = LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.from_returns(&[])?;
            Ok(state)
        })
        .unwrap();
    assert_eq!(state.value(), None);
    state.extend(&[0.0, -0.01, 0.0]).unwrap();
    assert_eq!(state.value(), Some(0.0));

    assert!(LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
    assert!(LongestWinningStreak::new(NanPolicy::Omit)
        .and_then(|mut state| {
            state.append(0.0)?;
            Ok(state)
        })
        .is_err());
}
