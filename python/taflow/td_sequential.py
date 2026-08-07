"""Tom DeMark TD Sequential setup counts (causal 4-bar comparison)."""
import numpy as np
class TDSequential:
    def __init__(self, close=None): self.reset(); self.extend(close) if close is not None else None
    def append(self, close):
        x=float(close); self._c.append(x); buy=sell=0
        if len(self._c)>4:
            if x<self._c[-5]: self._buy=min(self._buy+1,9); self._sell=0
            elif x>self._c[-5]: self._sell=min(self._sell+1,9); self._buy=0
            else: self._buy=self._sell=0
            buy,sell=self._buy,self._sell
        self._values.append((buy,sell)); return self._values[-1]
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return tuple(np.asarray(v,dtype=int) for v in zip(*self._values)) if self._values else (np.array([],dtype=int),)*2
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._c=[]; self._buy=self._sell=0; self._values=[]; return self
