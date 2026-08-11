use super::CompoundedGrowth;
use approx::assert_relative_eq;

#[test]
fn compounds_stably_and_tracks_total_loss() {
    let mut state = CompoundedGrowth::new();
    state.append(0.1).unwrap();
    state.append(-0.1).unwrap();
    assert_relative_eq!(state.growth_factor().unwrap(), 0.99, epsilon = 1e-15);
    assert!(state.append(-1.01).is_err());
    assert_eq!(state.len(), 2);
    state.append(-1.0).unwrap();
    state.append(0.5).unwrap();
    assert_eq!(state.growth_factor(), Some(0.0));
    assert!(state.is_total_loss());
    state.reset();
    assert!(state.is_empty());
}
