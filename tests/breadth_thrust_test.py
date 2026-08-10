from taflow import BreadthThrust
def test_lifecycle():
 x=BreadthThrust([1.,-1.],[0.,0.],[0.,0.],[0.,0.],2);assert x.value is not None;x.reset();assert x.value is None
