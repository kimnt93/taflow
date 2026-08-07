"""Persistent Know Sure Thing (bukosabino `ta` alignment)."""
from typing import Any
import numpy as np
from ._native import KstOperator as _Native
from ._series import as_float64_series


class Kst:
    def __init__(
        self,
        close: Any | None = None,
        roc1: int = 10,
        roc2: int = 15,
        roc3: int = 20,
        roc4: int = 30,
        sma1: int = 10,
        sma2: int = 10,
        sma3: int = 10,
        sma4: int = 15,
        signal: int = 9,
    ):
        self._state = _Native(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, signal)
        self.extend(close) if close is not None else None

    def append(self, close: float):
        self._state.append(close)
        return self

    def extend(self, close: Any):
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
