from taflow import CumulativeVolumeIndex
def test_lifecycle():
 x=CumulativeVolumeIndex([1.],[3.],[0.],[0.]);assert x.value==3.;x.reset();assert x.value is None
