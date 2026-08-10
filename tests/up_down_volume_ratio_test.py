from taflow import UpDownVolumeRatio
def test_lifecycle():
 x=UpDownVolumeRatio([1.,-1.],[2.,1.],[0.,0.],[0.,0.]);assert x.value==2.;x.reset();assert x.value is None
