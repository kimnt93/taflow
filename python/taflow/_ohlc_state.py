"""Shared native-backed adapter for high/low/close indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcStateAdapter:
    """Adapt a native three-input state without Python-side calculations

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    OhlcStateAdapter
        A persistent native-backed indicator adapter.
    """

    _native_cls = None
    _period_required = True

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        timeperiod: int = 14,
    ) -> None:
        """Create the native state and process initial OHLC data."""
        if self._period_required:
            self._state = self._native_cls(timeperiod)
        else:
            self._state = self._native_cls()
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "Self":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
