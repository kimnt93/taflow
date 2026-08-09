"""Shared native-backed adapters for volume indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcvStateAdapter:
    """Adapt a native high/low/close/volume state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    OhlcvStateAdapter
        A persistent native-backed indicator adapter.
    """

    _native_cls = None
    _constructor_args = ()

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        *parameters,
    ) -> None:
        """Create native state and process initial OHLCV data."""
        self._state = self._native_cls(*parameters)
        if any(value is not None for value in (high, low, close, volume)):
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.
        volume : float
            Current volume.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.
        volume : Any
            Chronological volume series.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
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


class CloseVolumeStateAdapter:
    """Adapt a native close/volume state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    CloseVolumeStateAdapter
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(
        self,
        close: Any,
        volume: Any,
    ) -> None:
        """Create native state and process initial close/volume data."""
        self._state = self._native_cls()
        if close is not None or volume is not None:
            self.extend(close, volume)

    def append(self, close: float, volume: float) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.
        volume : float
            Current volume.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.
        volume : Any
            Chronological volume series.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.extend(as_float64_series(close), as_float64_series(volume))
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
