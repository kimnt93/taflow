use super::GainLossState;

#[test]
fn separates_strict_gains_losses_and_breakevens() {
    let mut state = GainLossState::new();
    for value in [2.0, -3.0, 0.0, 4.0, -1.0] {
        state.append(value);
    }
    assert_eq!(state.gain_count(), 2);
    assert_eq!(state.loss_count(), 2);
    assert_eq!(state.breakeven_count(), 1);
    assert_eq!(state.gross_gain(), 6.0);
    assert_eq!(state.gross_loss(), -4.0);
    assert_eq!(state.average_gain(), Some(3.0));
    assert_eq!(state.average_loss(), Some(-2.0));
}
