"""Native-backed Fisher Transform adapter."""

from typing import Any

import numpy as np

from ._native import FisherTransformOperator as _Native
from ._series import as_float64_series


class FisherTransform:
    """Compute the causal Ehlers Fisher Transform of high/low midpoints.

    ``high`` and ``low`` are required equal-length chronological histories in
    that order and may both be empty for a fresh stream. ``timeperiod``
    defaults to 10. Rust normalizes each midpoint over its trailing high/low
    range, smooths and bounds it, then applies the Fisher logarithm; NaN warm-up
    and output alignment remain native. ``compute`` returns one float array,
    ``value`` is the latest scalar or ``None`` during warm-up, and lifecycle
    mutators return ``self``. The oracle is pandas-ta-classic ``fisher``; TA-Lib
    has no Fisher Transform function.
    """

    def __init__(
        self, high: Any, low: Any, timeperiod: int = 10
    ) -> None:
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(high, low)

    def append(self, high: float, low: float) -> "FisherTransform":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any) -> "FisherTransform":
        """Append equal-length high and low histories."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Fisher Transform history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest transform, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "FisherTransform":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length


__all__ = ["FisherTransform"]
