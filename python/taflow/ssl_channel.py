"""SSL Channel using rolling SMA(high/low) and a causal side state."""
import numpy as np
class SSLChannel:
    """Stateful SSLChannel indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high=None, low=None, close=None, length=10):
        self.length=int(length)
        if self.length<1: raise ValueError("length must be positive")
        self.reset()
        if close is not None: self.extend(high,low,close)
    def append(self, high, low, close):
        self._h.append(float(high)); self._l.append(float(low)); c=float(close); self._c.append(c); out=(np.nan,np.nan)
        if len(self._c)>=self.length:
            hh=sum(self._h[-self.length:])/self.length; ll=sum(self._l[-self.length:])/self.length
            self._hlv=1 if c>hh else -1 if c<ll else self._hlv
            out=(ll,hh) if self._hlv>0 else (hh,ll)
        self._values.append(out); return out
    def extend(self, high, low, close):
        for row in zip(high,low,close): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v,dtype=float) for v in zip(*self._values)) if self._values else (np.array([]),)*2
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._h=[]; self._l=[]; self._c=[]; self._hlv=1; self._values=[]; return self
