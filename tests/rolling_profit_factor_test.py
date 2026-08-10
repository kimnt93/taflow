import numpy as np
from taflow import RollingProfitFactor
def test_lifecycle():
    state=RollingProfitFactor(np.array([],dtype=float),3); state.extend([1.0,-1.0,0.0]); assert state.value is not None; state.reset(); assert len(state)==0
