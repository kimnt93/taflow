use super::CumulativeSumControlChart;

#[test]
fn cumulative_sum_control_chart_replays_after_reset() {
    let changes = [0.5, -0.5, 2.0, -1.0];
    let mut state = CumulativeSumControlChart::new(1.0).unwrap();
    let first: Vec<f64> = changes.iter().map(|&v| state.append(v)).collect();
    assert_eq!(first, vec![0.0, 0.0, 1.0, 0.0]);
    assert_eq!(state.value(), Some(0.0));
    state.reset();
    let second: Vec<f64> = changes.iter().map(|&v| state.append(v)).collect();
    assert_eq!(first, second);
}
