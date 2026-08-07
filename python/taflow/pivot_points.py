"""Classic session pivot levels."""
import numpy as np
class PivotPoints:
    def __init__(self, high=None, low=None, close=None, anchor=None): self.reset(); self.extend(high,low,close,anchor) if close is not None else None
    def append(self, high, low, close, anchor=False):
        if anchor and self._h is not None:
            p=(self._h+self._l+self._c)/3; self._levels=(p,2*p-self._l,2*p-self._h,p-(self._h-self._l),p+(self._h-self._l))
            self._h,self._l,self._c=float(high),float(low),float(close)
        else:
            self._h=float(high) if self._h is None else max(self._h,float(high)); self._l=float(low) if self._l is None else min(self._l,float(low)); self._c=float(close)
        self._v.append(self._levels); return self._levels
    def extend(self, high, low, close, anchor=None):
        if anchor is None: anchor=[False]*len(close)
        for row in zip(high,low,close,anchor): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._v)) if self._v else (np.array([]),)*5
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._h=self._l=self._c=None; self._levels=(np.nan,)*5; self._v=[]; return self
