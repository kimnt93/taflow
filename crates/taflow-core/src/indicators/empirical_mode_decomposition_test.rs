use super::empirical_mode_decomposition::EmpiricalModeDecomposition;
#[test]
fn lifecycle() {
    let mut s = EmpiricalModeDecomposition::new(3, 0.5).unwrap();
    s.append(1.0);
    s.append(2.0);
    assert!(s.append(3.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
