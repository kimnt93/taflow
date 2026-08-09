use super::rolling_maximum_drawdown::RollingMaximumDrawdown;
use crate::stream::StreamingIndicator;

#[test]
fn matches_peak_to_trough_windows() {
    let mut state = RollingMaximumDrawdown::new(3).unwrap();
    let actual = [100.0, 120.0, 90.0, 110.0, 80.0, 130.0]
        .into_iter()
        .map(|value| state.append(value))
        .collect::<Vec<_>>();
    assert_eq!(actual[..2], [None, None]);
    assert_eq!(actual[2], Some(0.25));
    assert_eq!(actual[3], Some(0.25));
    assert_eq!(actual[4], Some(30.0 / 110.0));
    assert_eq!(actual[5], Some(30.0 / 110.0));
}

#[test]
fn period_one_and_non_positive_peaks_yield_zero() {
    let mut one = RollingMaximumDrawdown::new(1).unwrap();
    assert_eq!(one.append(5.0), Some(0.0));
    assert_eq!(one.append(4.0), Some(0.0));

    let mut non_positive = RollingMaximumDrawdown::new(3).unwrap();
    assert_eq!(non_positive.append(-1.0), None);
    assert_eq!(non_positive.append(-2.0), None);
    assert_eq!(non_positive.append(-3.0), Some(0.0));
}

#[test]
fn ignores_non_finite_samples_without_advancing_warmup() {
    let mut state = RollingMaximumDrawdown::new(3).unwrap();
    assert_eq!(state.append(100.0), None);
    assert_eq!(state.append(f64::NAN), None);
    assert_eq!(state.append(90.0), None);
    assert_eq!(state.append(80.0), Some(0.2));
    assert_eq!(state.append(f64::INFINITY), Some(0.2));
}

#[test]
fn bulk_chunking_continuation_and_reset_are_invariant() {
    let values = [100.0, 105.0, 90.0, 95.0, 80.0, 120.0, 110.0];
    let mut batch = RollingMaximumDrawdown::new(4).unwrap();
    let mut batch_output = Vec::new();
    batch.extend_slice_into(&values, &mut batch_output);

    let mut chunked = RollingMaximumDrawdown::new(4).unwrap();
    let mut chunked_output = Vec::new();
    chunked.extend_slice_into(&values[..3], &mut chunked_output);
    chunked.extend_slice_into(&values[3..], &mut chunked_output);

    assert_eq!(batch_output.len(), chunked_output.len());
    for (left, right) in batch_output.iter().zip(&chunked_output) {
        assert!(left.to_bits() == right.to_bits() || (left.is_nan() && right.is_nan()));
    }
    assert_eq!(
        batch.value().unwrap().to_bits(),
        chunked.value().unwrap().to_bits()
    );
    assert_eq!(
        batch.append(100.0).unwrap().to_bits(),
        chunked.append(100.0).unwrap().to_bits()
    );

    batch.reset();
    assert_eq!(batch.value(), None);
    let mut replay = Vec::new();
    batch.extend_slice_into(&values, &mut replay);
    for (left, right) in batch_output.iter().zip(&replay) {
        assert!(left.to_bits() == right.to_bits() || (left.is_nan() && right.is_nan()));
    }
}

#[test]
fn rejects_zero_period() {
    assert!(RollingMaximumDrawdown::new(0).is_err());
}
