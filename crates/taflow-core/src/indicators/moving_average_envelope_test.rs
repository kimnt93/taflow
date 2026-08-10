use super::moving_average_envelope::MovingAverageEnvelope;
#[test]
fn lifecycle() {
    let mut s = MovingAverageEnvelope::new(2, 0.1).unwrap();
    s.append(1.0);
    assert!(s.append(2.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
