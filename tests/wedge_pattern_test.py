from taflow import WedgePattern
def test_lifecycle():
 x=WedgePattern([1.]*20,list(range(3,23)),list(range(1,21)),[2.]*20);assert x.value is not None;x.reset();assert x.value is None
