"""Opening range high/low and breakout flags."""
import numpy as np
class OpeningRange:
    def __init__(self, high=None, low=None, close=None, anchor=None, bars= opening if False else 30):
        self.bars=int(bars); self.reset()
        if close is not None: self.extend(high,low,close,anchor)
    def append(self, high, low, close, anchor=False):
        if anchor: self._n=0; self._hi=-np.inf; self._lo=np.inf
        if self._n<self.bars: self._hi=max(self._hi,float(high)); self._lo=min(self._lo,float(low)); self._n+=1
        b=1 if close>self._hi else -1 if close<self._lo else 0; self._v.append((self._hi,self._lo,b)); return self._v[-1]
    def extend(self, high, low, close, anchor=None):
        if anchor is None: anchor=[False]*len(close)
        for row in zip(high,low,close,anchor): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._v)) if self._v else (np.array([]),)*3
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._n=0; self._hi=-np.inf; self._lo=np.inf; self._v=[]; return self
