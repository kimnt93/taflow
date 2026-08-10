from taflow import MedianChannel
def test_lifecycle():
 x=MedianChannel([1.,2.,3.],3);assert x.value is not None;x.reset();assert x.value is None
