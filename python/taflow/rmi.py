"""Relative Momentum Index with Wilder smoothing."""
import numpy as np
class RelativeMomentumIndex:
    """Stateful RelativeMomentumIndex indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, close=None, length=14, mom=5):
        self.length=int(length); self.mom=int(mom)
        if self.length<1 or self.mom<1: raise ValueError("length and mom must be positive")
        self.reset()
        if close is not None: self.extend(close)
    def append(self, close):
        x=float(close); self._close.append(x); i=len(self._close)-1; out=np.nan
        if i>=self.mom:
            delta=x-self._close[i-self.mom]; up=max(delta,0.); dn=max(-delta,0.)
            if self._count<self.length: self._up+=up; self._dn+=dn; self._count+=1
            else: self._up=((self._up*(self.length-1))+up)/self.length; self._dn=((self._dn*(self.length-1))+dn)/self.length
            if self._count==self.length: out=100*self._up/(self._up+self._dn) if self._up+self._dn else 50.
        self._values.append(out); return out
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return np.asarray(self._values,dtype=float)
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._close=[]; self._up=self._dn=0.; self._count=0; self._values=[]; return self
