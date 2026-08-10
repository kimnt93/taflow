use super::up_down_volume_ratio::UpDownVolumeRatio;
#[test]
fn lifecycle() {
    let mut s = UpDownVolumeRatio::new().unwrap();
    assert_eq!(s.append(150., 50.), Some(3.));
    s.reset();
    assert!(s.value().is_none());
}
