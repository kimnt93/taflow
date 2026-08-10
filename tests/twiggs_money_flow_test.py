import numpy as np
from taflow import TwiggsMoneyFlow
def test_twiggs_money_flow_lifecycle():
    s=TwiggsMoneyFlow(np.array([],float),np.array([],float),np.array([],float),np.array([],float),2);s.extend([2,2],[1,1],[1.5,1.5],[10,10]);assert s.value is not None;s.reset();assert len(s)==0
