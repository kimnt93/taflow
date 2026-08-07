"""PMAX trend/risk line using EMA and Wilder ATR."""
import numpy as np
class PMAX:
    def __init__(self, high=None, low=None, close=None, length=10, multiplier=3.):
        self.length=int(length); self.multiplier=float(multiplier); self.reset()
        if close is not None: self.extend(high,low,close)
    def append(self, high, low, close):
        h,l,c=map(float,(high,low,close)); self._h.append(h); self._l.append(l); self._c.append(c); prev=self._c[-2] if len(self._c)>1 else c
        tr=max(h-l,abs(h-prev),abs(l-prev)); self._tr.append(tr); self._ema=c if self._ema is None else self._ema+2/(self.length+1)*(c-self._ema)
        atr=np.mean(self._tr[-self.length:]); up=self._ema+self.multiplier*atr; dn=self._ema-self.multiplier*atr
        self._up=up if self._up is None else min(up,self._up) if self._trend<0 else up; self._dn=dn if self._dn is None else max(dn,self._dn) if self._trend>0 else dn
        if self._trend>0 and c<self._dn: self._trend=-1
        elif self._trend<0 and c>self._up: self._trend=1
        out=self._dn if self._trend>0 else self._up; self._values.append((out,self._trend)); return self._values[-1]
    def extend(self, high, low, close):
        for row in zip(high,low,close): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._values)) if self._values else (np.array([]),)*2
    @property
    def value(self): return self._values[-1] if self._values else None
    def reset(self): self._h=[]; self._l=[]; self._c=[]; self._tr=[]; self._ema=None; self._up=self._dn=None; self._trend=1; self._values=[]; return self
