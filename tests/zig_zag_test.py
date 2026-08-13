from taflow import ZigZag
def test_lifecycle():
    x=ZigZag().extend([2.0], [1.0]);assert len(x)==1;x.reset().append(3.0,2.0);assert len(x)==1
