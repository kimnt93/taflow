use super::premium_discount::PremiumDiscount;

#[test]
fn reset_replay_matches() {
    let input: Vec<f64> = (0..64).map(|index| 100.0 + index as f64).collect();
    let mut state = PremiumDiscount::new(20).unwrap();
    let mut zones = Vec::new();
    let mut equilibrium = Vec::new();
    state.extend_slice_into(&input, &mut zones, &mut equilibrium);
    let final_value = state.value();
    state.reset();
    let mut replay_zones = Vec::new();
    let mut replay_equilibrium = Vec::new();
    state.extend_slice_into(&input, &mut replay_zones, &mut replay_equilibrium);
    assert_eq!(zones, replay_zones);
    assert_eq!(equilibrium, replay_equilibrium);
    assert_eq!(state.value(), final_value);
}
