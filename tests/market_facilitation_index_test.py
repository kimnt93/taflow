import numpy as np
from taflow import MarketFacilitationIndex
def test_market_facilitation_index_lifecycle():
    s=MarketFacilitationIndex();s.extend([2],[1],[10]);assert s.value is not None;s.reset();assert len(s)==0
