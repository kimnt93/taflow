from taflow import PercentAboveMovingAverage
def test_lifecycle():
 x=PercentAboveMovingAverage([0.],[0.],[0.],[0.],[.4]);assert x.value==40.;x.reset();assert x.value is None
