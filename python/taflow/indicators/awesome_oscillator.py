"""Native-backed Awesome Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import AwesomeOscillator as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class AwesomeOscillator:
    """Compute fast-minus-slow averages of the median price.

    ``high`` and ``low`` are required equal-length chronological series in
    that order and may both be empty for a fresh stream. ``fast`` defaults to
    5 and ``slow`` to 34. Rust owns median-price formation, smoothing, warm-up,
    and aligned history. ``compute`` returns one float array, ``value`` is the
    latest scalar or ``None`` during warm-up, and lifecycle mutators return
    ``self``. The oracle is pandas-ta-classic ``ao``.
    """

    def __init__(
        self, high: Any, low: Any, fast: int = 5, slow: int = 34
    ) -> None:
        self._state = _Native(int(fast), int(slow))
        self.extend(high, low)

    def append(self, high: float, low: float) -> "AwesomeOscillator":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "AwesomeOscillator":
        """Append equal-length high and low histories."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Awesome Oscillator history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "AwesomeOscillator":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)


__all__ = ["AwesomeOscillator"]
