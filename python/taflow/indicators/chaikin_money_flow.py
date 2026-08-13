"""Canonical native-backed Chaikin Money Flow adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import ChaikinMoneyFlowOperator as _Native
from .._series import as_float64_series


class ChaikinMoneyFlow:
    """Chaikin Money Flow over high, low, close, and volume."""

    def __init__(
        self, period: int = 20
    ) -> None:
        self._state = _Native(int(period))

    def append(
        self, high: float, low: float, close: float, volume: float
    ) -> "ChaikinMoneyFlow":
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any
    ) -> "ChaikinMoneyFlow":
        arrays = [as_float64_series(v) for v in (high, low, close, volume)]
        if not all(a.shape == arrays[0].shape for a in arrays):
            raise ValueError("OHLCV series must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "ChaikinMoneyFlow":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
