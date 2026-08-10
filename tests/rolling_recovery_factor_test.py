import numpy as np
from taflow import RollingRecoveryFactor
def test_lifecycle():
    state=RollingRecoveryFactor(np.array([],dtype=float),3); state.extend([1.0,2.0,1.5]); assert state.value is not None; state.reset(); assert len(state)==0
