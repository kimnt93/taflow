"""Ehlers Laguerre RSI."""
import numpy as np
class LaguerreRelativeStrengthIndex:
    """Stateful LaguerreRelativeStrengthIndex indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, close=None, gamma=.5):
        if not 0<=gamma<1: raise ValueError("gamma must be in [0,1)")
        self.gamma=float(gamma); self.reset()
        if close is not None: self.extend(close)
    def append(self, close):
        x=float(close); g=self.gamma; a,b,c,d=self._l
        l0=(1-g)*x+g*a; l1=-g*l0+a+g*b; l2=-g*l1+b+g*c; l3=-g*l2+c+g*d
        cu=sum(max(u-v,0) for u,v in ((l0,l1),(l1,l2),(l2,l3))); cd=sum(max(v-u,0) for u,v in ((l0,l1),(l1,l2),(l2,l3)))
        r=cu/(cu+cd) if cu+cd else 0.; self._l=(l0,l1,l2,l3); self._values.append(r); return r
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return np.asarray(self._values,dtype=float)
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._l=(0.,0.,0.,0.); self._values=[]; return self
