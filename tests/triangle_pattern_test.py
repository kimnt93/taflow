from taflow import TrianglePattern
def test_lifecycle():
 x=TrianglePattern([1.]*20,list(range(30,10,-1)),list(range(20)),[2.]*20);assert x.value is not None;x.reset();assert x.value is None
