"""Python adapter for Ehlers empirical mode decomposition."""

from typing import Any

import numpy as np

from .._native import EmpiricalModeDecomposition as _Native
from .._series import as_float64_series


class EmpiricalModeDecomposition:
    """Compute Ehlers' bandpass-and-envelope empirical-mode line.

    Rust applies a resonant bandpass centred on ``period``, measures rolling
    peaks and valleys over ``round(period * fraction)`` bars, smooths the two
    envelopes, and returns the centred bandpass. Warm-up positions are ``NaN``.
    The definition maps to Wickra ``EmpiricalModeDecomposition``.

    Args:
        prices: Required chronological price history.
        period: Bandpass centre period. Defaults to 20.
        fraction: Peak/valley window as a fraction of ``period``. Defaults to
            0.5 and must be finite and in ``(0, 1]``.

    Raises:
        ValueError: If ``period`` or ``fraction`` is invalid.
    """

    def __init__(self, period: int = 20, fraction: float = 0.5) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, fraction)

    def append(self, price: float) -> "EmpiricalModeDecomposition":
        """Append one price and return this adapter."""
        self._state.append(float(price))
        return self

    def extend(self, prices: Any) -> "EmpiricalModeDecomposition":
        """Append a chronological price series through native Rust."""
        self._state.extend(as_float64_series(prices))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest EMD line, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned EMD history with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "EmpiricalModeDecomposition":
        """Clear bandpass and envelope state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-price count delegated to native state."""
        return len(self._state)
