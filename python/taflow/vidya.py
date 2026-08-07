"""Chande VIDYA (CMO-modulated EMA), causal and stateful."""
import numpy as np

class VIDYA:
    def __init__(self, close=None, length=14, alpha=None):
        self.length=int(length); self.alpha=2/(self.length+1) if alpha is None else float(alpha)
        if self.length<1 or not 0<self.alpha<=1: raise ValueError("invalid length/alpha")
        self.reset()
        if close is not None: self.extend(close)
    def append(self, close):
        x=float(close); self._close.append(x); n=min(self.length,len(self._close)-1)
        if self._value is None: self._value=x
        elif n:
            d=np.diff(self._close[-(n+1):]); up=float(d[d>0].sum()); dn=float(-d[d<0].sum()); cmo=(up-dn)/(up+dn) if up+dn else 0.; self._value=self.alpha*abs(cmo)*x+(1-self.alpha*abs(cmo))*self._value
        self._values.append(self._value); return self._value
    def extend(self, close):
        for x in close: self.append(x)
        return self
    def compute(self): return np.asarray(self._values,dtype=float)
    @property
    def value(self): return self._value
    def reset(self): self._close=[]; self._values=[]; self._value=None; return self
