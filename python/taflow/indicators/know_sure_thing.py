"""Canonical native-backed Know Sure Thing adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import KnowSureThingOperator as _Native
from .._series import as_float64_series


class KnowSureThing:
    """Native-backed Know Sure Thing oscillator with KST and signal outputs.

    ``close`` is a required chronological price series. ``roc1`` through
    ``roc4`` default to 10, 15, 20, 30; ``sma1`` through ``sma4`` default to
    10, 10, 10, 15; and ``signal`` defaults to 9. ``compute`` returns the
    aligned ``(kst, signal)`` NumPy arrays with NaN warm-up positions, while
    ``value`` returns the latest tuple or ``None`` before output is warm.
    ``append``, ``extend``, and ``reset`` mutate and return this adapter. The
    pandas-ta-classic oracle is ``know_sure_thing``; TAFlow preserves the
    bukosabino/ta KST scaling variant.
    """

    def __init__(
        self,
        close: Any,
        roc1: int = 10,
        roc2: int = 15,
        roc3: int = 20,
        roc4: int = 30,
        sma1: int = 10,
        sma2: int = 10,
        sma3: int = 10,
        sma4: int = 15,
        signal: int = 9,
    ) -> None:
        self._state = _Native(
            int(roc1),
            int(roc2),
            int(roc3),
            int(roc4),
            int(sma1),
            int(sma2),
            int(sma3),
            int(sma4),
            int(signal),
        )
        self.extend(close)

    def append(self, close: float) -> "KnowSureThing":
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "KnowSureThing":
        values = as_float64_series(close)
        self._state.extend(values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        return self._state.value

    def reset(self) -> "KnowSureThing":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
