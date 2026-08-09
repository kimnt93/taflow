use super::BreakOfStructureChangeOfCharacter;

#[test]
fn break_of_structure_change_of_character_resets() {
    let bars: Vec<(f64, f64, f64)> = (0..128)
        .map(|i| {
            let close = 100.0 + (i as f64 * 0.17).sin() * 4.0;
            (close + 1.0, close - 1.0, close)
        })
        .collect();
    let mut state = BreakOfStructureChangeOfCharacter::new(3).unwrap();
    let first: Vec<_> = bars
        .iter()
        .map(|&(h, l, c)| state.append(h, l, c))
        .collect();
    state.reset();
    let second: Vec<_> = bars
        .iter()
        .map(|&(h, l, c)| state.append(h, l, c))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.bos.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.bos.to_bits()).collect::<Vec<_>>()
    );
}
