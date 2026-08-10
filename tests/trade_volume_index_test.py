import numpy as np
from taflow import TradeVolumeIndex
def test_trade_volume_index_lifecycle():
    s=TradeVolumeIndex(np.array([],float),np.array([],float));s.extend([1,2],[10,10]);assert s.value is not None;s.reset();assert len(s)==0
