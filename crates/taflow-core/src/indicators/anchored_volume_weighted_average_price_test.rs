use super::anchored_volume_weighted_average_price::{
    AnchoredVolumeWeightedAveragePrice, AnchoredVolumeWeightedAveragePriceValue,
};

fn assert_same_bits(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len());
    for (&left, &right) in left.iter().zip(right) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
}

#[test]
fn anchored_volume_weighted_average_price_matches_weighted_moments() {
    let mut state = AnchoredVolumeWeightedAveragePrice::new(1.0).unwrap();
    let first = state.append(12.0, 9.0, 12.0, 2.0, true);
    assert_eq!(
        first,
        AnchoredVolumeWeightedAveragePriceValue {
            volume_weighted_average_price: 11.0,
            upper_band: 11.0,
            lower_band: 11.0,
        }
    );

    let second = state.append(15.0, 12.0, 15.0, 2.0, false);
    assert_eq!(second.volume_weighted_average_price, 12.5);
    assert_eq!(second.upper_band, 14.0);
    assert_eq!(second.lower_band, 11.0);
    assert_eq!(state.value(), Some(second));

    let reset_value = state.append(21.0, 18.0, 21.0, 4.0, true);
    assert_eq!(reset_value.volume_weighted_average_price, 20.0);
    assert_eq!(reset_value.upper_band, 20.0);
    assert_eq!(reset_value.lower_band, 20.0);
    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(12.0, 9.0, 12.0, 2.0, true), first);
}

#[test]
fn anchored_volume_weighted_average_price_bulk_is_chunk_invariant() {
    let high = [12.0, 15.0, 18.0, 21.0, 24.0];
    let low = [9.0, 12.0, 15.0, 18.0, 21.0];
    let close = [12.0, 15.0, 18.0, 21.0, 24.0];
    let volume = [2.0, 3.0, 5.0, 7.0, 11.0];
    let anchor = [true, false, false, true, false];

    let mut batch = AnchoredVolumeWeightedAveragePrice::new(2.0).unwrap();
    let (mut batch_average, mut batch_upper, mut batch_lower) =
        (Vec::new(), Vec::new(), Vec::new());
    batch
        .extend_slices_into(
            &high,
            &low,
            &close,
            &volume,
            &anchor,
            &mut batch_average,
            &mut batch_upper,
            &mut batch_lower,
        )
        .unwrap();

    let mut chunked = AnchoredVolumeWeightedAveragePrice::new(2.0).unwrap();
    let (mut chunked_average, mut chunked_upper, mut chunked_lower) =
        (Vec::new(), Vec::new(), Vec::new());
    for range in [0..2, 2..5] {
        chunked
            .extend_slices_into(
                &high[range.clone()],
                &low[range.clone()],
                &close[range.clone()],
                &volume[range.clone()],
                &anchor[range],
                &mut chunked_average,
                &mut chunked_upper,
                &mut chunked_lower,
            )
            .unwrap();
    }
    assert_same_bits(&chunked_average, &batch_average);
    assert_same_bits(&chunked_upper, &batch_upper);
    assert_same_bits(&chunked_lower, &batch_lower);
    assert_eq!(chunked.value(), batch.value());
}

#[test]
fn anchored_volume_weighted_average_price_validates_before_mutation() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(AnchoredVolumeWeightedAveragePrice::new(invalid).is_err());
    }

    let mut state = AnchoredVolumeWeightedAveragePrice::new(1.0).unwrap();
    let (mut average, mut upper, mut lower) = (Vec::new(), Vec::new(), Vec::new());
    assert!(state
        .extend_slices_into(
            &[1.0],
            &[],
            &[1.0],
            &[1.0],
            &[true],
            &mut average,
            &mut upper,
            &mut lower,
        )
        .is_err());
    assert_eq!(state.value(), None);
    assert!(average.is_empty());
    assert!(upper.is_empty());
    assert!(lower.is_empty());
}
