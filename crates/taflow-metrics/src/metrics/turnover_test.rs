use super::turnover::Turnover;
use crate::NanPolicy;

#[test]
fn computes_mean_one_way_weight_turnover() {
    let mut metric = Turnover::new(NanPolicy::Omit).unwrap();
    metric.from_weights(&[]).unwrap();
    assert_eq!(metric.append(0.0).unwrap(), None);
    assert_eq!(metric.append(0.5).unwrap(), Some(0.5));
    assert_eq!(metric.append(-0.25).unwrap(), Some(0.625));
    assert_eq!(metric.append(0.25).unwrap(), Some(7.0 / 12.0));
}

#[test]
fn scalar_chunk_reset_and_cached_compute_are_invariant() {
    let values = [0.0, 0.4, 0.1, -0.2, 0.0];
    let mut scalar = Turnover::new(NanPolicy::Omit).unwrap();
    scalar.from_weights(&[]).unwrap();
    for value in values {
        scalar.append(value).unwrap();
    }
    let expected = scalar.compute();
    assert_eq!(expected, scalar.compute());

    let mut chunked = Turnover::new(NanPolicy::Omit).unwrap();
    chunked.from_weights(&[]).unwrap();
    chunked.extend(&values[..2]).unwrap();
    chunked.extend(&values[2..]).unwrap();
    assert_eq!(chunked.compute(), expected);
    chunked.reset();
    assert!(chunked.is_empty());
    assert_eq!(chunked.extend(&values).unwrap(), expected);
}

#[test]
fn missing_values_and_invalid_values_follow_policy() {
    let mut omit = Turnover::new(NanPolicy::Omit).unwrap();
    omit.from_weights(&[]).unwrap();
    omit.extend(&[0.0, f64::NAN, 0.5]).unwrap();
    assert_eq!(omit.len(), 2);
    assert_eq!(omit.compute(), Some(0.5));
    assert!(omit.append(f64::INFINITY).is_err());
    assert_eq!(omit.len(), 2);

    let mut raise = Turnover::new(NanPolicy::Raise).unwrap();
    raise.from_weights(&[]).unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert!(raise.is_empty());
}

#[test]
fn warmup_requires_two_valid_weights() {
    let mut metric = Turnover::new(NanPolicy::Omit).unwrap();
    metric.from_weights(&[]).unwrap();
    assert_eq!(metric.compute(), None);
    assert_eq!(metric.append(1.5).unwrap(), None);
}
