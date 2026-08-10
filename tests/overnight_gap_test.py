from taflow import OvernightGap
def test_lifecycle():
    x=OvernightGap([1.0,3.0],[1.0,3.0],[1.0,3.0],[2.0,3.0],[1.0,1.0],[0,86_400_000_000_000]);assert len(x)==2;x.reset();assert x.value is None
