"""Shared native-backed adapter for high/low/close indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class OhlcStateAdapter:
    """Adapt a native three-input state without Python-side calculations."""

    _native_cls = None
    _period_required = True

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        timeperiod: int = 14,
    ) -> None:
        """Create the native state and optionally process initial OHLC data."""
        if self._period_required:
            self._state = self._native_cls(timeperiod)
        else:
            self._state = self._native_cls()
        self._values: list[float] = []
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> object:
        """Append one OHLC bar and update the native state."""
        value = self._state.append(float(high), float(low), float(close))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> object:
        """Append aligned high, low, and close histories to native state."""
        values = self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned native output history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest native output, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history."""
        self._state.reset()
        self._values.clear()
        return self
