"""Public Jurik-like adaptive moving average reconstruction (not proprietary JMA)."""
import numpy as np
class JMA:
    def __init__(self, close=None, length=7, phase=0):
        self.length=int(length); self.phase=float(phase)
        if self.length<1: raise ValueError("length must be positive")
        self.reset()
        if close is not None: self.extend(close)
    def append(self, close):
        x=float(close); self._close.append(x)
        if self._value is None: self._value=x
        else:
            a=2/(self.length+1); vol=np.mean(np.abs(np.diff(self._close[-min(len(self._close),self.length+1):]))) if len(self._close)>1 else 0
            dev=abs(x-self._value); adapt=a*(1+min(dev/(vol+1e-12),1)); self._value=self._value+min(adapt,1)*(x-self._value)
        self._values.append(self._value); return self._value
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return np.asarray(self._values,dtype=float)
    @property
    def value(self): return self._value
    def reset(self): self._close=[]; self._value=None; self._values=[]; return self
