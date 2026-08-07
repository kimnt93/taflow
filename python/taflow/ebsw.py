"""Ehlers Even Better Sinewave-style detrended cycle oscillator."""
import numpy as np
class EvenBetterSinewave:
    def __init__(self, close=None, length=40): self.length=int(length); self.reset(); self.extend(close) if close is not None else None
    def append(self, close):
        x=float(close); self._c.append(x); hp=0 if len(self._c)<3 else .5*(1-.5)*(x-2*self._c[-2]+self._c[-3]) + 1.0*self._hp[-1] if self._hp else 0
        v=hp if not self._v else .5*hp+.5*self._v[-1]; self._hp.append(hp); self._v.append(v); return v
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return np.asarray(self._v,dtype=float)
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._c=[]; self._hp=[]; self._v=[]; return self
