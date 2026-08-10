from typing import Any
import numpy as np
from .._native import HilbertDominantCycle as _Native
from .._series import as_float64_series

class HilbertDominantCycle:
    """Causal Hilbert-transform dominant-cycle period with native warm-up."""
    def __init__(self, prices: Any) -> None: self._state = _Native(); self.extend(prices)
    def append(self, price: float) -> "HilbertDominantCycle": self._state.append(float(price)); return self
    def extend(self, prices: Any) -> "HilbertDominantCycle": self._state.extend(as_float64_series(prices)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "HilbertDominantCycle": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
