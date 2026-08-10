from taflow import ArmsIndex
def test_lifecycle():
 x=ArmsIndex([1.,-1.],[2.,1.],[0.,0.],[0.,0.]);assert x.value is not None;x.reset();assert x.value is None
