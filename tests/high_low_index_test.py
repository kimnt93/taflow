from taflow import HighLowIndex
def test_lifecycle():
 x=HighLowIndex([0.,0.],[0.,0.],[1.,0.],[0.,1.],2);assert x.value==50.;x.reset();assert x.value is None
