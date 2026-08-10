import numpy as np
from taflow import DemandIndex
def test_lifecycle():
    empty=np.array([],dtype=float); state=DemandIndex(empty,empty,empty,empty); state.extend([2.0],[0.0],[1.0],[10.0]); assert state.value is not None; state.reset(); assert len(state)==0
