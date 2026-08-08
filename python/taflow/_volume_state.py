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
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
        *parameters: int,
    ) -> None:
        """Create native state and optionally process initial OHLCV data."""
        self._state = self._native_cls(*parameters)
        if any(value is not None for value in (high, low, close, volume)):
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> object:
        """Append one OHLCV bar and update native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> object:
        """Append aligned OHLCV histories to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return latest native output, or ``None`` during warm-up

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

    def __init__(self, close: Any | None = None, volume: Any | None = None) -> None:
        """Create native state and optionally process initial close/volume data."""
        self._state = self._native_cls()
        if close is not None or volume is not None:
            self.extend(close, volume)

    def append(self, close: float, volume: float) -> object:
        """Append one close/volume observation and update native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> object:
        """Append aligned close and volume histories to native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(as_float64_series(close), as_float64_series(volume))
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return latest native output, or ``None`` during warm-up

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
        return self

    def __len__(self) -> int:
        return len(self._state)
