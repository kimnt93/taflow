use super::PairedMoments;
use approx::assert_relative_eq;

#[test]
fn computes_covariance_variance_and_correlation() {
    let mut state = PairedMoments::new();
    for (primary, benchmark) in [(2.0, 1.0), (4.0, 2.0), (6.0, 3.0)] {
        state.append(primary, benchmark);
    }
    assert_eq!(state.means(), Some((4.0, 2.0)));
    assert_relative_eq!(state.covariance(1).unwrap(), 2.0);
    assert_relative_eq!(state.benchmark_variance(1).unwrap(), 1.0);
    assert_relative_eq!(state.primary_variance(1).unwrap(), 4.0);
    assert_relative_eq!(state.correlation().unwrap(), 1.0);
    state.reset();
    assert!(state.is_empty());
}
