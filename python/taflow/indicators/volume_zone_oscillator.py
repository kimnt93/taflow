"""Native-backed Volume Zone Oscillator adapter."""
from typing import Any
import numpy as np
from .._native import VolumeZoneOscillator as _Native
from .._series import as_float64_series

class VolumeZoneOscillator:
    """Causal signed-volume percentage oscillator from close and volume.

    Required aligned inputs are ``close`` and ``volume`` in that order;
    ``timeperiod`` defaults to 14. Rust owns state and arithmetic, while
    lifecycle mutators return ``self`` and ``compute`` returns float64. Oracle
    mapping: Wickra ``VZO``/``VolumeZoneOscillator``.
    """
    def __init__(self, close: Any, volume: Any, timeperiod: int = 14) -> None: self._state = _Native(int(timeperiod)); self.extend(close, volume)
    def append(self, close: float, volume: float) -> "VolumeZoneOscillator": self._state.append(float(close), float(volume)); return self
    def extend(self, close: Any, volume: Any) -> "VolumeZoneOscillator":
        arrays=(as_float64_series(close),as_float64_series(volume))
        if len(arrays[0])!=len(arrays[1]): raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "VolumeZoneOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__=["VolumeZoneOscillator"]
