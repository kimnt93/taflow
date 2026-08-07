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
        _open: Any | None = None,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Create native state and optionally process initial OHLC data."""
        self._state = StatefulBop()
        self._values: list[float] = []
        if any(value is not None for value in (_open, high, low, close)):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float) -> object:
        """Append one OHLC bar and update native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        value = self._state.append(_open, high, low, close)
        self._values.append(value)
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> object:
        """Append aligned OHLC histories to native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        values = self._state.extend(
            as_float64_series(_open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        self._values.extend(values.tolist())
        return self

    def compute(self) -> object:
        """Return aligned native Balance of Power history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        import numpy as np

        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest native value..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values.clear()
        return self
