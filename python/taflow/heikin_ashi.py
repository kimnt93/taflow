"""Causal Heikin–Ashi OHLC transform."""
import numpy as np

class HeikinAshi:
    def __init__(self, open=None, high=None, low=None, close=None):
        self.reset()
        if open is not None: self.extend(open, high, low, close)
    def append(self, open, high, low, close):
        c=(float(open)+float(high)+float(low)+float(close))/4
        o=(float(open)+float(close))/2 if self._prev_open is None else (self._prev_open+self._prev_close)/2
        v=(o,max(float(high),o,c),min(float(low),o,c),c); self._prev_open,self._prev_close=o,c; self._values.append(v); return v
    def extend(self, open, high, low, close):
        for row in zip(open,high,low,close): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._values)) if self._values else (np.array([]),)*4
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._prev_open=self._prev_close=None; self._values=[]; return self
