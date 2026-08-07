"""Shared native-backed price-transform lifecycle adapters."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcPriceState:
    """Adapt a native four-input OHLC state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    OhlcPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(
        self,
        _open: Any | None = None,
        high: object = None,
        low: object = None,
        close: object = None,
    ) -> None:
        """Create native state and optionally process initial OHLC data."""
        self._state = self._native_cls()
        self._values: list[float] = []
        if any(value is not None for value in (_open, high, low, close)):
            self.extend(_open, high, low, close)

    def append(self, _open: object, high: object, low: object, close: object) -> object:
        """Append one OHLC bar to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._values.append(self._state.append(_open, high, low, close))
        return self

    def extend(self, _open: object, high: object, low: object, close: object) -> object:
        """Append aligned OHLC histories to native state

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

    def compute(self) -> np.ndarray:
        """Return aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return latest native output

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values.clear()
        return self


class HlcPriceState:
    """Adapt a native three-input HLC state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    HlcPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(
        self, high: Any | None = None, low: object = None, close: object = None
    ) -> None:
        """Create native state and optionally process initial HLC data."""
        self._state = self._native_cls()
        self._values: list[float] = []
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> object:
        """Append one HLC bar to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._values.append(self._state.append(high, low, close))
        return self

    def extend(self, high: object, low: object, close: object) -> object:
        """Append aligned HLC histories to native state

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
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        self._values.extend(values.tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return latest native output

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values.clear()
        return self


class HlPriceState:
    """Adapt a native two-input high/low state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    HlPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(self, high: Any | None = None, low: object = None) -> None:
        """Create native state and optionally process initial high/low data."""
        self._state = self._native_cls()
        self._values: list[float] = []
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: object, low: object) -> object:
        """Append one high/low observation to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._values.append(self._state.append(high, low))
        return self

    def extend(self, high: object, low: object) -> object:
        """Append aligned high/low histories to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        values = self._state.extend(as_float64_series(high), as_float64_series(low))
        self._values.extend(values.tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return latest native output

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values.clear()
        return self
