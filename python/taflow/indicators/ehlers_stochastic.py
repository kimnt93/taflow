"""Public adapter for native Ehlers Stochastic."""

from typing import Any
import numpy as np
from .._native import EhlersStochastic as _Native
from .._series import as_float64_series


class EhlersStochastic:
    """Normalize a roofing-filtered price into a smoothed ``[-1, 1]`` range.

    Rust applies the standard 10/48 Roofing Filter, computes stochastic
    position over ``period`` filtered bars, and averages two raw oscillator
    values. Warm-up is represented by ``NaN``. It maps to Wickra
    ``EhlersStochastic``.

    Args:
        values: Initial chronological price history.
        period: Positive stochastic lookback, default 10.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, values: Any, period: int = 10) -> None:
        """Initialize native filter state and process the initial history."""
        self._state = _Native(period)
        self.extend(values)

    def append(self, value: float) -> "EhlersStochastic":
        """Append one price and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "EhlersStochastic":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned oscillator output with warm-up ``NaN`` values."""
        return self._state.compute()

    def reset(self) -> "EhlersStochastic":
        """Reset native filter and oscillator state, then return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
