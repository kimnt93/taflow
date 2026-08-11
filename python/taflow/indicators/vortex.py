"""Canonical native-backed Vortex adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import VortexOperator as _Native
from .._series import as_float64_series


class Vortex:
    """Vortex positive and negative directional ratios over OHLC series."""

    def __init__(self, high: Any, low: Any, close: Any, window: int = 14) -> None:
        self._state = _Native(int(window))
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "Vortex":
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "Vortex":
        arrays = [as_float64_series(v) for v in (high, low, close)]
        if not (arrays[0].shape == arrays[1].shape == arrays[2].shape):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        return self._state.value

    def reset(self) -> "Vortex":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
