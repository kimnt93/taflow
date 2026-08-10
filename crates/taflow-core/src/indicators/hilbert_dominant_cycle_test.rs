use super::hilbert_dominant_cycle::HilbertDominantCycle;
#[test]
fn lifecycle() {
    let mut s = HilbertDominantCycle::new().unwrap();
    // Wickra's Hilbert transform contract withholds values for 50 bars while
    // the recursive phasor estimates settle.
    for i in 0..60 {
        s.append(i as f64);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
