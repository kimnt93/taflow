from taflow import McClellanSummationIndex
def test_lifecycle():
 x=McClellanSummationIndex([1.],[0.],[0.],[0.]);assert len(x)==1;x.reset();assert x.value is None
