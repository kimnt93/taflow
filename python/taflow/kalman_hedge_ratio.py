"""Online Kalman estimate of the hedge ratio in ``y = alpha + beta*x``."""
from typing import Any
import numpy as np
from ._native import KalmanHedgeRatioOperator as _Native
from ._series import as_float64_series


class KalmanHedgeRatio:
    """Stateful KalmanHedgeRatio indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(
        self,
        x: Any | None = None,
        y: Any | None = None,
        delta: float = 1e-4,
        observation_variance: float = 1e-3,
    ):
        self._state = _Native(delta, observation_variance)
        if x is not None or y is not None:
            self.extend(x, y)

    def append(self, x: float, y: float):
        self._state.append(x, y)
        return self

    def extend(self, x: Any, y: Any):
        self._state.extend(as_float64_series(x), as_float64_series(y))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    @property
    def alpha(self):
        return self._state.alpha

    @property
    def innovation(self):
        return self._state.innovation

    @property
    def std(self):
        return self._state.std

    def reset(self):
        self._state.reset()
        return self
