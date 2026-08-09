"""Native-backed Garman-Klass volatility adapter."""

from typing import Any

import numpy as np

from .._native import GarmanKlassOperator as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class GarmanKlass:
    """Compute rolling Garman-Klass OHLC volatility.

    ``_open``, ``high``, ``low``, and ``close`` are required aligned
    chronological histories; four empty arrays create a fresh stream.
    ``timeperiod`` defaults to 20 and controls the trailing mean. Rust owns
    the formula ``0.5*ln(H/L)^2 - (2*ln(2)-1)*ln(C/O)^2``, rolling window, and
    NaN warm-up. ``compute`` returns one aligned float array and ``value`` the
    latest scalar. Lifecycle mutators return ``self``. The independent oracle
    is the Garman-Klass definition used by pandas-ta-classic.
    """

    def __init__(
        self,
        _open: Any,
        high: Any,
        low: Any,
        close: Any,
        timeperiod: int = 20,
    ) -> None:
        """Initialize and process aligned OHLC histories.

        Parameters
        ----------
        _open, high, low, close : object
            Required aligned OHLC histories; empty arrays create a fresh state.
        timeperiod : int, default 20
            Positive trailing window length in bars.
        """
        self._state = _Native(int(timeperiod))
        self.extend(_open, high, low, close)

    def append(
        self, _open: float, high: float, low: float, close: float
    ) -> "GarmanKlass":
        """Append one OHLC bar in open/high/low/close order."""
        self._state.append(float(_open), float(high), float(low), float(close))
        return self

    def extend(
        self, _open: Any, high: Any, low: Any, close: Any
    ) -> "GarmanKlass":
        """Append aligned OHLC histories without mutating on mismatch."""
        arrays = tuple(as_float64_series(value) for value in (_open, high, low, close))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("_open, high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling volatility history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest volatility, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "GarmanKlass":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of OHLC bars processed by Rust."""
        return adapter_length(self)


__all__ = ["GarmanKlass"]
