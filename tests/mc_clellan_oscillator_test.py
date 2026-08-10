from taflow import McClellanOscillator
def test_lifecycle():
 x=McClellanOscillator([1.],[0.],[0.],[0.]);assert len(x)==1;x.reset();assert x.value is None
