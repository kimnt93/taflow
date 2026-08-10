use super::up_down_volume_ratio::UpDownVolumeRatio;
#[test]
fn lifecycle() {
    let mut s = UpDownVolumeRatio::new().unwrap();
    s.append(1., 2., 0., 0.);
    assert_eq!(s.append(-1., 1., 0., 0.), Some(2.));
    s.reset();
    assert!(s.value().is_none());
}
