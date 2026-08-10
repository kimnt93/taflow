import numpy as np
from taflow import EmpiricalModeDecomposition
def test_lifecycle():
    x=EmpiricalModeDecomposition(np.arange(5.0),3);assert len(x)==5;x.reset().append(1.0);assert len(x)==1
