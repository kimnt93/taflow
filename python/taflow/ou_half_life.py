"""Ornstein-Uhlenbeck mean-reversion half-life of a price series."""
from typing import Any
import numpy as np
from ._native import OuHalfLifeOperator as _Native
from ._series import as_float64_series


class OrnsteinUhlenbeckHalfLife:
    """Stateful OrnsteinUhlenbeckHalfLife indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, price: Any | None = None, timeperiod: int = 20):
        self._state = _Native(timeperiod)
        self.extend(price) if price is not None else None

    def append(self, price: float):
        self._state.append(price)
        return self

    def extend(self, price: Any):
        self._state.extend(as_float64_series(price))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
