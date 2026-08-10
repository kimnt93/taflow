"""Native-backed Keltner-channel adapter."""

from typing import Any

import numpy as np

from .._native import KeltnerChannelsOperator as _Native
from .._series import as_float64_series


class KeltnerChannels:
    """Compute EMA-based causal Keltner channels.

    ``high``, ``low``, and ``close`` are required equal-length chronological
    histories in that order and may all be empty for a fresh stream.
    ``timeperiod`` defaults to 20 and ``multiplier`` to 2.0. The native state
    uses a seeded EMA of typical price and Wilder average true range;
    ``compute`` returns
    ``(upper, middle, lower)`` arrays. Lifecycle mutators return ``self`` and
    ``value`` exposes the latest tuple or ``None``. The formula, output order,
    and warm-up match Wickra ``Keltner``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        timeperiod: int = 20,
        multiplier: float = 2.0,
    ) -> None:
        self._state = _Native(int(timeperiod), float(multiplier))
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "KeltnerChannels":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "KeltnerChannels":
        """Append equal-length OHLC histories."""
        arrays = tuple(as_float64_series(series) for series in (high, low, close))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned ``(upper, middle, lower)`` arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest channel tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "KeltnerChannels":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["KeltnerChannels"]
