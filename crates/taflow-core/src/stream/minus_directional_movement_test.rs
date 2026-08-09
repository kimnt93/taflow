use super::minus_directional_movement::MinusDirectionalMovement;

#[test]
fn reset_replays_minus_directional_movement() {
    let bars: Vec<(f64, f64)> = (0..64)
        .map(|index| (100.0 + index as f64, 99.0 + index as f64))
        .collect();
    let mut state = MinusDirectionalMovement::new(14).unwrap();
    let first: Vec<_> = bars.iter().map(|&(h, l)| state.append(h, l)).collect();
    state.reset();
    let second: Vec<_> = bars.iter().map(|&(h, l)| state.append(h, l)).collect();
    assert_eq!(first, second);
}
