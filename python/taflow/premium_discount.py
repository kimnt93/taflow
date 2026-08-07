"""Premium/discount zones relative to a rolling swing midpoint."""
import numpy as np
class PremiumDiscount:
    def __init__(self, close=None, window=20): self.window=int(window); self.reset(); self.extend(close) if close is not None else None
    def append(self, close):
        x=float(close); self._c.append(x); lo=min(self._c[-self.window:]); hi=max(self._c[-self.window:]); eq=(hi+lo)/2; z=1 if x>eq else -1 if x<eq else 0; self._v.append((z,eq)); return self._v[-1]
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._v)) if self._v else (np.array([]),)*2
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._c=[]; self._v=[]; return self
