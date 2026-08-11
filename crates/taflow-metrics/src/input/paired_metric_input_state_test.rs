use super::{MetricInputKind, NanPolicy, PairedMetricInputState};
use approx::assert_relative_eq;

#[test]
fn omits_nan_pairs_without_advancing_either_equity_path() {
    let mut state = PairedMetricInputState::new(
        MetricInputKind::Equity,
        MetricInputKind::Equity,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_eq!(state.append(100.0, 200.0).unwrap(), None);
    assert_eq!(state.append(110.0, f64::NAN).unwrap(), None);
    let (primary, benchmark) = state.append(120.0, 220.0).unwrap().unwrap();
    assert_relative_eq!(primary, 0.2);
    assert_relative_eq!(benchmark, 0.1);
    assert_eq!(state.len(), 1);
}

#[test]
fn errors_and_length_mismatch_are_transactional() {
    let mut state = PairedMetricInputState::new(
        MetricInputKind::Returns,
        MetricInputKind::Returns,
        NanPolicy::Raise,
    )
    .unwrap();
    assert!(state.append(0.1, f64::NAN).is_err());
    assert!(state.is_empty());
    assert!(state.extend_slices(&[0.1, 0.2], &[0.1], |_, _| {}).is_err());
    assert!(state.is_empty());
    assert_eq!(state.append(0.1, 0.2).unwrap(), Some((0.1, 0.2)));
}

#[test]
fn rejects_different_semantic_domains() {
    assert!(PairedMetricInputState::new(
        MetricInputKind::Returns,
        MetricInputKind::Equity,
        NanPolicy::Omit,
    )
    .is_err());
}
