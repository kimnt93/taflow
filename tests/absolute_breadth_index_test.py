from taflow import AbsoluteBreadthIndex
def test_lifecycle():
 x=AbsoluteBreadthIndex([-2.],[1.],[0.],[0.]);assert x.value==2.;x.reset();assert x.value is None
