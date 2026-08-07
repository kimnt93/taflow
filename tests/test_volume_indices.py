import numpy as np
from taflow import Nvi, Pvi
def reference(close, volume, negative):
    out=np.empty(len(close)); out[0]=1000.0
    for i in range(1,len(close)):
        out[i]=out[i-1]
        if (volume[i]<volume[i-1])==negative: out[i]*=1+(close[i]-close[i-1])/close[i-1]
    return out
def test_volume_indices():
    close=np.arange(1.,101.); volume=np.where(np.arange(100)%2,1.,2.)
    np.testing.assert_allclose(Nvi(close,volume).compute(),reference(close,volume,True))
    np.testing.assert_allclose(Pvi(close,volume).compute(),reference(close,volume,False))
