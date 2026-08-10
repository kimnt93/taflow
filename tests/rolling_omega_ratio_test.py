import numpy as np
from taflow import RollingOmegaRatio
def test_lifecycle():
    state=RollingOmegaRatio(np.array([],dtype=float),3); assert state.append(1.0) is state; state.extend([1.0,-1.0]); assert state.value is not None; state.reset(); assert len(state)==0
