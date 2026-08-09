"""Canonical Balance of Power adapter."""

from typing import Any

from ._native import StatefulBop
from ._series import as_float64_series


class BalanceOfPower:
    """Compute Balance of Power from aligned OHLC histories

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    BalanceOfPower
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        _open: Any,
        high: Any,
        low: Any,
        close: Any,
    ) -> None:
        """Create native state and process initial OHLC data."""
        self._state = StatefulBop()
        if any(value is not None for value in (_open, high, low, close)):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float) -> "BalanceOfPower":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _open : float
            Current open price.
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.

        Returns
        -------
        BalanceOfPower
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(_open, high, low, close)
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "BalanceOfPower":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _open : Any
            Chronological open price series.
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.

        Returns
        -------
        BalanceOfPower
            This indicator, for fluent chaining."""
        self._state.extend(
            as_float64_series(_open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        return self

    def compute(self) -> object:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        import numpy as np

        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "BalanceOfPower":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        BalanceOfPower
            This indicator, for fluent chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
