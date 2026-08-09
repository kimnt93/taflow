"""Canonical native-backed Hilbert Transform Trendline adapter."""

from typing import Any

import numpy as np

from .._native import HilbertTransformTrendline as _NativeHilbertTransformTrendline
from .._series import as_float64_series


class HilbertTransformTrendline:
    """Compute TA-Lib ``HT_TRENDLINE`` from a chronological price series.

    ``values`` is required and may be empty for a fresh stream. The Rust state
    handles Hilbert warm-up; aligned ``compute`` output contains NaN values
    before the first trendline, while scalar ``value`` is ``None``. Lifecycle
    mutators are fluent. Oracle mapping: ``HilbertTransformTrendline`` ⇔
    TA-Lib ``HT_TRENDLINE``.
    """

    def __init__(self, values: Any) -> None:
        self._state = _NativeHilbertTransformTrendline()
        self.extend(values)

    def append(self, value: float) -> "HilbertTransformTrendline":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "HilbertTransformTrendline":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "HilbertTransformTrendline":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["HilbertTransformTrendline"]
