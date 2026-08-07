"""Rolling Fibonacci retracement levels."""
import numpy as np
class FibonacciRetracement:
    def __init__(self, close=None, window=120):
        if int(window)<1: raise ValueError("window must be positive")
        self.window=int(window); self.reset()
        if close is not None: self.extend(close)
    def append(self, close):
        self._close.append(float(close)); lo=min(self._close[-self.window:]); hi=max(self._close[-self.window:]); span=hi-lo
        v=tuple(hi-span*r for r in (0,.236,.382,.5,.618,.786,1)); self._values.append(v); return v
    def extend(self, close):
        for v in close: self.append(v)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._values)) if self._values else (np.array([]),)*7
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._close=[]; self._values=[]; return self
