"""Persistent Mesa Adaptive Moving Average interface."""

from typing import Any

import numpy as np

from .._native import MesaAdaptiveMovingAverage as _NativeMesaAdaptiveMovingAverage
from .._series import as_float64_series


class MesaAdaptiveMovingAverage:
    """Compute the MAMA and FAMA pair in persistent native Rust state.

    ``_input`` is required. ``fastlimit`` and ``slowlimit`` default to 0.5 and
    0.05, matching TA-Lib ``MAMA``. History contains both aligned outputs.
    """

    def __init__(self, fastlimit: float = 0.5, slowlimit: float = 0.05) -> None:
        self._state = _NativeMesaAdaptiveMovingAverage(fastlimit, slowlimit)

    def append(self, _input: float) -> "MesaAdaptiveMovingAverage":
        """Append one value and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "MesaAdaptiveMovingAverage":
        """Append a chronological series and return this indicator."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned MAMA and FAMA histories."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest MAMA/FAMA pair or None during warm-up."""
        return self._state.value

    def reset(self) -> "MesaAdaptiveMovingAverage":
        """Reset native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
