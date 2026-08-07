"""Session volume-profile levels using a bounded fixed-bin histogram."""
import numpy as np
class SessionVolumeLevels:
    def __init__(self, high=None, low=None, close=None, volume=None, anchor=None, bins=24, value_area=.7):
        self.bins=int(bins); self.value_area=float(value_area); self.reset()
        if close is not None: self.extend(high,low,close,volume,anchor)
    def append(self, high, low, close, volume, anchor=False):
        if anchor or self._lo is None: self._lo=float(low); self._hi=float(high); self._hist=np.zeros(self.bins); self._step=max((self._hi-self._lo)/self.bins,1e-12)
        self._lo=min(self._lo,float(low)); self._hi=max(self._hi,float(high)); idx=min(self.bins-1,max(0,int((float(close)-self._lo)/self._step))); self._hist[idx]+=float(volume)
        poc=int(np.argmax(self._hist)); total=self._hist.sum(); target=total*self.value_area; left=right=poc; acc=self._hist[poc]
        while acc<target and (left>0 or right<self.bins-1):
            if left==0: right+=1
            elif right==self.bins-1: left-=1
            elif self._hist[left-1]>=self._hist[right+1]: left-=1
            else: right+=1
            acc=self._hist[left:right+1].sum()
        out=((poc+.5)*self._step+self._lo,(right+.5)*self._step+self._lo,(left+.5)*self._step+self._lo); self._v.append(out); return out
    def extend(self, high, low, close, volume, anchor=None):
        if anchor is None: anchor=[False]*len(close)
        for row in zip(high,low,close,volume,anchor): self.append(*row)
        return self
    def compute(self): return tuple(np.asarray(v) for v in zip(*self._v)) if self._v else (np.array([]),)*3
    @property
    def value(self): return self._v[-1] if self._v else None
    def reset(self): self._lo=self._hi=None; self._hist=None; self._step=1.; self._v=[]; return self
