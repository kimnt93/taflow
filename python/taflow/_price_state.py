"""Shared native-backed price-transform lifecycle adapters."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcPriceState:
    """Adapt a native four-input OHLC state.

    The constructor creates an empty state; inputs are supplied to ``extend``.

    Returns
    -------
    OhlcPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(self) -> None:
        """Create an empty native OHLC state."""
        self._state = self._native_cls()

    def append(self, _open: float, high: float, low: float, close: float) -> "Self":
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

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "Self":
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
    def value(self) -> float | None:
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
    """Adapt a native three-input HLC state.

    The constructor creates an empty state; inputs are supplied to ``extend``.

    Returns
    -------
    HlcPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(self) -> None:
        """Create an empty native HLC state."""
        self._state = self._native_cls()

    def append(self, high: float, low: float, close: float) -> "Self":
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

    def extend(self, high: Any, low: Any, close: Any) -> "Self":
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
    def value(self) -> float | None:
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
    """Adapt a native two-input high/low state.

    The constructor creates an empty state; inputs are supplied to ``extend``.

    Returns
    -------
    HlPriceState
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(self) -> None:
        """Create an empty native high/low state."""
        self._state = self._native_cls()

    def append(self, high: float, low: float) -> "Self":
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

    def extend(self, high: Any, low: Any) -> "Self":
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
    def value(self) -> float | None:
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
