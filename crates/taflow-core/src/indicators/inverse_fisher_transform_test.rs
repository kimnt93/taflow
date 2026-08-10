use super::inverse_fisher_transform::InverseFisherTransform;
#[test]
fn lifecycle() {
    let mut s = InverseFisherTransform::new(1.0).unwrap();
    assert!(s.append(1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
