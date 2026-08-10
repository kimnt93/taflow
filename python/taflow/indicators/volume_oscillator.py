"""Native-backed volume oscillator adapter."""
from typing import Any
import numpy as np
from .._native import VolumeOscillator as _Native
from .._series import as_float64_series

class VolumeOscillator:
    """Fast-minus-slow EMA oscillator of required chronological volume.

    ``fast`` and ``slow`` default to 5 and 10. Rust owns EMA state and warm-up;
    lifecycle mutators return ``self`` and ``compute`` returns one float64 array.
    Oracle mapping: Wickra ``VolumeOscillator``.
    """
    def __init__(self, volume: Any, fast: int = 5, slow: int = 10) -> None: self._state = _Native(int(fast), int(slow)); self.extend(volume)
    def append(self, volume: float) -> "VolumeOscillator": self._state.append(float(volume)); return self
    def extend(self, volume: Any) -> "VolumeOscillator": self._state.extend(as_float64_series(volume)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "VolumeOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["VolumeOscillator"]
