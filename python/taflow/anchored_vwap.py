"""Anchored VWAP with running standard-deviation bands."""
import numpy as np

class AnchoredVolumeWeightedAveragePrice:
    """Stateful AnchoredVolumeWeightedAveragePrice indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high=None, low=None, close=None, volume=None, anchor=None, stdev=1.0):
        self.stdev=float(stdev); self.reset()
        if close is not None: self.extend(high,low,close,volume,anchor)
    def append(self, high, low, close, volume, anchor=False):
        if anchor or self._n == 0: self._n=self._pv=self._v=self._p2v=0.0
        p=(float(high)+float(low)+float(close))/3; v=float(volume); self._pv+=p*v; self._p2v+=p*p*v; self._v+=v; self._n+=1
        mean=self._pv/self._v if self._v else np.nan; var=max(self._p2v/self._v-mean*mean,0) if self._v else np.nan
        x=(mean, mean+self.stdev*np.sqrt(var), mean-self.stdev*np.sqrt(var)); self._values.append(x); return x
    def extend(self, high, low, close, volume, anchor=None):
        if anchor is None: anchor=[False]*len(close)
        for row in zip(high,low,close,volume,anchor): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._values)) if self._values else (np.array([]),)*3
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._n=self._pv=self._v=self._p2v=0.; self._values=[]; return self
