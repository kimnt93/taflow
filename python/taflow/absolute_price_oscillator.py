"""Canonical Absolute Price Oscillator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulApo
from ._series import as_float64_series


class AbsolutePriceOscillator:
    """Compute the absolute price oscillator with native moving averages.

    Parameters
    ----------
    _input : array-like, optional
        Initial input history.
    fastperiod, slowperiod : int
        Fast and slow moving-average periods.
    moving_average_type : int, default 0
        Moving-average selector compatible with :class:`taflow.MaType`.
    """

    def __init__(
        self,
        _input: Any | None = None,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native APO state and optionally process initial inputs."""
        self._state = StatefulApo(fastperiod, slowperiod, int(moving_average_type))
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> object:
        """Append one input value to native APO state."""
        value = self._state.append(float(_input))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, _input: Any) -> object:
        """Append an aligned input history to native APO state."""
        values = self._state.extend(as_float64_series(_input))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned APO history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest APO value."""
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history."""
        self._state.reset()
        self._values.clear()
        return self
