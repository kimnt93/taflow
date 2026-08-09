"""Persistent Accumulation/Distribution Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import AccumulationDistributionOscillator as _NativeAccumulationDistributionOscillator
from .._series import as_float64_series


class AccumulationDistributionOscillator:
    """Compute fast minus slow EMA of the A/D line in persistent Rust state.

    The constructor requires aligned chronological high, low, close, and volume
    series. Pass four empty arrays for a fresh streaming state. ``fastperiod``
    and ``slowperiod`` default to 3 and 10 and must both be at least 2. Outputs
    are NaN for ``max(fastperiod, slowperiod) - 1`` warm-up bars. The definition
    and first-value EMA seeds map to TA-Lib ``ADOSC``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        fastperiod: int = 3,
        slowperiod: int = 10,
    ) -> None:
        self._state = _NativeAccumulationDistributionOscillator(fastperiod, slowperiod)
        self.extend(high, low, close, volume)

    def append(
        self, high: float, low: float, close: float, volume: float
    ) -> "AccumulationDistributionOscillator":
        """Append one high/low/close/volume tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any
    ) -> "AccumulationDistributionOscillator":
        """Append aligned price and volume histories and return this indicator."""
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned ``float64`` oscillator history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "AccumulationDistributionOscillator":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
