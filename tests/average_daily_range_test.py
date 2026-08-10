from taflow import AverageDailyRange
def test_lifecycle():
    x=AverageDailyRange([1.0,2.0],[3.0,4.0],[1.0,2.0],[2.0,3.0],[1.0,1.0],[0,86_400_000_000_000],2);assert len(x)==2;x.reset();assert x.value is None
