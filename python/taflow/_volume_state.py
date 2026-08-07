"""Shared native-backed adapters for volume indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcvStateAdapter:
    """Adapt a native high/low/close/volume state."""

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
        self._values: list[float] = []
        if any(value is not None for value in (high, low, close, volume)):
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float):
        """Append one OHLCV bar and update native state."""
        value = self._state.append(float(high), float(low), float(close), float(volume))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any):
        """Append aligned OHLCV histories to native state."""
        values = self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self):
        """Return latest native output, or ``None`` during warm-up."""
        return self._state.value

    def reset(self):
        """Reset native state and accumulated output history."""
        self._state.reset()
        self._values.clear()
        return self


class CloseVolumeStateAdapter:
    """Adapt a native close/volume state."""

    _native_cls = None

    def __init__(self, close: Any | None = None, volume: Any | None = None) -> None:
        """Create native state and optionally process initial close/volume data."""
        self._state = self._native_cls()
        self._values: list[float] = []
        if close is not None or volume is not None:
            self.extend(close, volume)

    def append(self, close: float, volume: float):
        """Append one close/volume observation and update native state."""
        value = self._state.append(float(close), float(volume))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, close: Any, volume: Any):
        """Append aligned close and volume histories to native state."""
        values = self._state.extend(as_float64_series(close), as_float64_series(volume))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned native output history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self):
        """Return latest native output, or ``None`` during warm-up."""
        return self._state.value

    def reset(self):
        """Reset native state and accumulated output history."""
        self._state.reset()
        self._values.clear()
        return self
