from taflow import QuartileBands
def test_lifecycle():
 x=QuartileBands([1.,2.,3.],3);assert x.value is not None;x.reset();assert x.value is None
