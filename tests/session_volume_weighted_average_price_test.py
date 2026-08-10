from taflow import SessionVolumeWeightedAveragePrice
def test_lifecycle():
    x=SessionVolumeWeightedAveragePrice([1.0],[2.0],[0.0],[1.0],[2.0],[0]);assert len(x)==1;x.reset();assert x.value is None
