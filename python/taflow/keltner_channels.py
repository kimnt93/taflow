"""Persistent Keltner channel adapter."""

from typing import Any
import numpy as np
from ._native import KeltnerChannelsOperator as _Native
from ._series import as_float64_series


class KeltnerChannels:
    """Causal EMA-based Keltner channel.

    ``high``, ``low``, and ``close`` are required aligned histories (empty
    arrays create a fresh stream). ``timeperiod`` defaults to 20 and
    ``multiplier`` defaults to 2.0. The channel uses EMA typical price and
    EMA high-low range, returning ``(upper, middle, lower)`` arrays. Outputs
    are causal from the first bar; lifecycle methods are fluent and ``value``
    returns the latest tuple or ``None``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        timeperiod: int = 20,
        multiplier: float = 2.0,
    ) -> None:
        """Create the native state and replay aligned OHLC histories."""
        self._state = _Native(timeperiod, multiplier)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "KeltnerChannels":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "KeltnerChannels":
        """Append aligned OHLC histories and return this adapter."""
        arrays = tuple(as_float64_series(series) for series in (high, low, close))
        if not (arrays[0].shape == arrays[1].shape == arrays[2].shape):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return ``(upper, middle, lower)`` aligned output arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest ``(upper, middle, lower)`` tuple or ``None``."""
        return self._state.value

    def reset(self) -> "KeltnerChannels":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state.compute()[0])
