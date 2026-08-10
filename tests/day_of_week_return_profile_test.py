from taflow import DayOfWeekReturnProfile
def test_lifecycle():
    x=DayOfWeekReturnProfile([1.0,1.0],[1.0,2.0],[1.0,1.0],[1.0,2.0],[1.0,1.0],[0,1]);assert len(x)==2;x.reset();assert x.value is None
