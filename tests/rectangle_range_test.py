from taflow import RectangleRange
def test_lifecycle():
 x=RectangleRange([1.]*20,[1.01]*20,[.99]*20,[1.]*20);assert x.value is not None;x.reset();assert x.value is None
