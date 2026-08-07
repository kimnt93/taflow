from typing import Any
import numpy as np
from ._native import BarsSinceOperator,ValueWhenOperator,HighestSinceOperator,LowestSinceOperator,SignalDelayOperator,PositionHoldOperator,EntryExitOperator
from ._series import as_float64_series
class BarsSince:
    def __init__(self,condition:Any|None=None):self._state=BarsSinceOperator();self.extend(condition) if condition is not None else None
    def append(self,condition:bool):self._state.append(condition);return self
    def extend(self,condition:Any):self._state.extend(np.asarray(condition,dtype=bool));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
def _make(native,name):
    class State:
        def __init__(self,condition:Any|None=None,input:Any|None=None):self._state=native();self.extend(condition,input) if condition is not None or input is not None else None
        def append(self,condition:bool,input:float):self._state.append(condition,input);return self
        def extend(self,condition:Any,input:Any):self._state.extend(np.asarray(condition,dtype=bool),as_float64_series(input));return self
        def compute(self)->np.ndarray:return self._state.compute()
        @property
        def value(self):return self._state.value
        def reset(self):self._state.reset();return self
    State.__name__=name
    return State
ValueWhen=_make(ValueWhenOperator,'ValueWhen'); HighestSince=_make(HighestSinceOperator,'HighestSince'); LowestSince=_make(LowestSinceOperator,'LowestSince')
class SignalDelay:
    def __init__(self,timeperiod:int,input:Any|None=None):self._state=SignalDelayOperator(timeperiod);self.extend(input) if input is not None else None
    def append(self,input:float):self._state.append(input);return self
    def extend(self,input:Any):self._state.extend(as_float64_series(input));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
class PositionHold:
    def __init__(self,input:Any|None=None):self._state=PositionHoldOperator();self.extend(input) if input is not None else None
    def append(self,input:float):self._state.append(input);return self
    def extend(self,input:Any):self._state.extend(as_float64_series(input));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
class EntryExit:
    def __init__(self,entry:Any|None=None,exit:Any|None=None):self._state=EntryExitOperator();self.extend(entry,exit) if entry is not None or exit is not None else None
    def append(self,entry:bool,exit:bool):self._state.append(entry,exit);return self
    def extend(self,entry:Any,exit:Any):self._state.extend(np.asarray(entry,dtype=bool),np.asarray(exit,dtype=bool));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
