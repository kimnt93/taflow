import numpy as np
from taflow import MovingAverageEnvelope
def test_moving_average_envelope_lifecycle():
    s=MovingAverageEnvelope(np.array([],float),2,.1);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
