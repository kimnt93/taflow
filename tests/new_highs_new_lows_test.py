from taflow import NewHighsNewLows
def test_lifecycle():
 x=NewHighsNewLows([0.],[0.],[3.],[1.]);assert x.value==2.;x.reset();assert x.value is None
