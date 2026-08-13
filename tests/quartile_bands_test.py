from taflow import QuartileBands
def test_lifecycle():
 x=QuartileBands(3).extend([1.,2.,3.]);assert x.value is not None;x.reset();assert x.value is None
