from taflow import CupAndHandle
def test_lifecycle():
 x=CupAndHandle().extend([1.]*20, [2.]*20, [0.]*20, list(range(20)));assert x.value is not None;x.reset();assert x.value is None
