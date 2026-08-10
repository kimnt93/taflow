from taflow import FlagPennant
def test_lifecycle():
 x=FlagPennant([1.]*20,[3.]*20,[0.]*20,list(range(20)));assert x.value is not None;x.reset();assert x.value is None
