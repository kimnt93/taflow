from taflow import BullishPercentIndex
def test_lifecycle():
 x=BullishPercentIndex([0.],[0.],[0.],[0.],[.5]);assert x.value==50.;x.reset();assert x.value is None
