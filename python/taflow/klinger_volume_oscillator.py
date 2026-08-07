"""Klinger volume oscillator (causal fast/slow EMA of signed volume force)."""
import numpy as np
class KlingerVolumeOscillator:
    """Stateful KlingerVolumeOscillator indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high=None, low=None, close=None, volume=None, fast=34, slow=55, signal=13):
        self.fast=int(fast); self.slow=int(slow); self.signal=int(signal); self.reset()
        if close is not None: self.extend(high,low,close,volume)
    def append(self, high, low, close, volume):
        h,l,c,v=map(float,(high,low,close,volume)); tp=(h+l+c)/3; trend=1 if self._tp is None or tp>=self._tp else -1; force=trend*v*(h-l)
        self._tp=tp; self._ef=force if self._ef is None else self._ef+2/(self.fast+1)*(force-self._ef); self._es=force if self._es is None else self._es+2/(self.slow+1)*(force-self._es); ko=self._ef-self._es; self._sig=ko if self._sig is None else self._sig+2/(self.signal+1)*(ko-self._sig); self._v.append((ko,self._sig)); return self._v[-1]
    def extend(self, high, low, close, volume):
        for row in zip(high,low,close,volume): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._v)) if self._v else (np.array([]),)*2
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._tp=self._ef=self._es=self._sig=None; self._v=[]; return self
