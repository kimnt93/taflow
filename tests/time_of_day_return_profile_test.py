from taflow import TimeOfDayReturnProfile
def test_lifecycle():
    x=TimeOfDayReturnProfile([1.0,1.0],[1.0,2.0],[1.0,1.0],[1.0,2.0],[1.0,1.0],[0,1]);assert len(x)==2;x.reset();assert x.value is None
