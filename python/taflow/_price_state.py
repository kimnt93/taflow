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
        _open: Any,
        high: object,
        low: object,
        close: object,
    ) -> None:
        """Create native state and process initial OHLC data."""
        self._state = self._native_cls()
        if any(value is not None for value in (_open, high, low, close)):
            self.extend(_open, high, low, close)

    def append(self, _open: object, high: object, low: object, close: object) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _open : object
            Current open price.
        high : object
            Current high price.
        low : object
            Current low price.
        close : object
            Current close price.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(_open, high, low, close)
        return self

    def extend(self, _open: object, high: object, low: object, close: object) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _open : object
            Chronological open price series.
        high : object
            Chronological high price series.
        low : object
            Chronological low price series.
        close : object
            Chronological close price series.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.extend(
            as_float64_series(_open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
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
        self,
        high: Any,
        low: object,
        close: object,
    ) -> None:
        """Create native state and process initial HLC data."""
        self._state = self._native_cls()
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : object
            Current high price.
        low : object
            Current low price.
        close : object
            Current close price.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(high, low, close)
        return self

    def extend(self, high: object, low: object, close: object) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : object
            Chronological high price series.
        low : object
            Chronological low price series.
        close : object
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

    def __init__(
        self,
        high: Any,
        low: object,
    ) -> None:
        """Create native state and process initial high/low data."""
        self._state = self._native_cls()
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: object, low: object) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : object
            Current high price.
        low : object
            Current low price.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(high, low)
        return self

    def extend(self, high: object, low: object) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : object
            Chronological high price series.
        low : object
            Chronological low price series.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.extend(as_float64_series(high), as_float64_series(low))
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
