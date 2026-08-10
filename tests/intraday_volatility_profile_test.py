from taflow import IntradayVolatilityProfile
def test_lifecycle():
 x=IntradayVolatilityProfile([1.,1.],[1.,2.],[1.,1.],[1.,2.],[1.,1.],[0,1]);assert len(x)==2;x.reset();assert x.value is None
