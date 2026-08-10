from taflow import VolumeByTimeProfile
def test_lifecycle():
 x=VolumeByTimeProfile([1.],[1.],[1.],[1.],[4.],[0]);assert x.value==4.;x.reset();assert len(x)==0
