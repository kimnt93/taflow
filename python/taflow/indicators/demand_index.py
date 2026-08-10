"""Native-backed Demand Index adapter."""
from typing import Any
import numpy as np
from .._native import DemandIndex as _Native
from .._series import as_float64_series

class DemandIndex:
    """Causal price/volume pressure value from high, low, close, volume.

    Required aligned inputs are consumed in that order. Rust owns the demand
    calculation and lifecycle; ``append``, ``extend``, and ``reset`` return
    ``self`` and ``compute`` returns one float64 array. Oracle mapping: Wickra
    ``DemandIndex``.
    """
    def __init__(self, high: Any, low: Any, close: Any, volume: Any) -> None: self._state=_Native(); self.extend(high,low,close,volume)
    def append(self, high: float, low: float, close: float, volume: float) -> "DemandIndex": self._state.append(float(high),float(low),float(close),float(volume)); return self
    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "DemandIndex":
        arrays=tuple(as_float64_series(x) for x in (high,low,close,volume))
        if len({len(x) for x in arrays})!=1: raise ValueError("high, low, close, and volume must have equal lengths")
        self._state.extend(*arrays); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "DemandIndex": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__=["DemandIndex"]
