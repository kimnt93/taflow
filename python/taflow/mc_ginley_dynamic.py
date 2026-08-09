"""Canonical native-backed McGinley Dynamic adapter."""
from typing import Any
import numpy as np
from ._native import McGinleyDynamicOperator as _Native
from ._series import as_float64_series


class McGinleyDynamic:
    """Native-backed causal McGinley Dynamic moving average.

    ``close`` is a required chronological price series. ``length`` (default
    10) and ``c`` (default 1.0) are positive recurrence parameters. ``compute``
    returns one aligned NumPy array, ``value`` is ``None`` before the first
    append, and ``append``, ``extend``, and ``reset`` mutate and return this
    adapter. The independent pandas-ta-classic oracle is ``mcginley``.
    """
    def __init__(self, close: Any, length: int = 10, c: float = 1.0) -> None:
        self._state = _Native(int(length), float(c)); self._length = 0; self.extend(close)
    def append(self, close: float) -> "McGinleyDynamic":
        self._state.append(float(close)); self._length += 1; return self
    def extend(self, close: Any) -> "McGinleyDynamic":
        values = as_float64_series(close); self._state.extend(values); self._length += len(values); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "McGinleyDynamic": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
