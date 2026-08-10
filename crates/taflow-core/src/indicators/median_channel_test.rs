use super::median_channel::MedianChannel;
#[test]
fn lifecycle() {
    let mut s = MedianChannel::new(3, 2.).unwrap();
    s.append(1.);
    s.append(2.);
    assert!(s.append(3.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
