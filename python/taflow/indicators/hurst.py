"""Causal rolling rescaled-range Hurst estimate."""

from typing import Any
import numpy as np
from .._native import HurstOperator as _Native
from .._series import as_float64_series


class Hurst:
    """Estimate the rolling Hurst exponent with rescaled-range analysis.

    Each trailing ``timeperiod`` window is divided into ``chunks`` equal
    segments. The native Rust state regresses log rescaled ranges on log chunk
    sizes, matching Wickra ``HurstExponent``. Output is ``NaN`` until a complete
    window is available. ``append``, ``extend``, and ``reset`` return this
    instance; ``value`` exposes the latest scalar and ``compute`` returns the
    aligned history.

    Args:
        _input: Required chronological price series. An empty series creates a
            fresh streaming state.
        timeperiod: Trailing window length. Must be at least ``chunks * 2``.
        chunks: Number of rescaled-range segments. Must be at least two.

    Raises:
        ValueError: If the native configuration is invalid.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 20,
        chunks: int = 4,
    ) -> None:
        """Initialize the estimator and process the supplied price history."""
        self._state = _Native(timeperiod, chunks)
        self.extend(_input)

    def append(self, _input: float) -> "Hurst":
        """Append one price observation and return this estimator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "Hurst":
        """Append a chronological price series and return this estimator."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Hurst-exponent history as a NumPy array."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest estimate, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Hurst":
        """Restore fresh native state and return this estimator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-observation count from native state."""
        return len(self._state)
