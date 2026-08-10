from taflow import SessionRange
def test_lifecycle():
    x=SessionRange([1.0],[3.0],[1.0],[2.0],[1.0],[0]);assert x.value==2.0;x.reset();assert len(x)==0
