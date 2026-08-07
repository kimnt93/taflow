import numpy as np
from taflow import McGinleyDynamic
def test_mcginley_reference_and_chunks():
    x=100+np.sin(np.arange(100.)*.2)*4
    expected=np.empty(100);expected[0]=x[0]
    for i in range(1,100):
        d=1*10*(x[i]/expected[i-1])**4; d=max(d,1e-10)
        expected[i]=expected[i-1]+(x[i]-expected[i-1])/d
    full=McGinleyDynamic(x).compute();np.testing.assert_allclose(full,expected,atol=1e-12)
    chunked=McGinleyDynamic()
    for i in range(0,100,13):chunked.extend(x[i:i+13])
    np.testing.assert_array_equal(chunked.compute(),full)
