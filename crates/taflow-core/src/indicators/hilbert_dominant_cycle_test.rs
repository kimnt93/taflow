use super::hilbert_dominant_cycle::HilbertDominantCycle;
#[test]
fn lifecycle() {
    let mut s = HilbertDominantCycle::new().unwrap();
    for i in 0..40 {
        s.append(i as f64);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
