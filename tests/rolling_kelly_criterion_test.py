import numpy as np
from taflow import RollingKellyCriterion
def test_lifecycle():
    state=RollingKellyCriterion(np.array([],dtype=float),3); state.extend([1.0,-1.0,0.5]); assert state.value is not None; state.reset(); assert len(state)==0
