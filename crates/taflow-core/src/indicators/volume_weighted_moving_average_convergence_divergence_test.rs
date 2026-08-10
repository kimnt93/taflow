use super::volume_weighted_moving_average_convergence_divergence::VolumeWeightedMovingAverageConvergenceDivergence;
#[test]
fn lifecycle() {
    let mut s = VolumeWeightedMovingAverageConvergenceDivergence::new(2, 3).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 2.0).is_none());
    s.reset();
    assert!(s.value().is_none());
}
