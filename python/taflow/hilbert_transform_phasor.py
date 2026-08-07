"""Persistent Hilbert Transform phasor components (HT_PHASOR)."""

from typing import Any
import numpy as np

from ._native import HilbertTransformPhasor as _Native
from ._series import as_float64_series


class HilbertTransformPhasor:
    """Stateful HilbertTransformPhasor indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, _input: Any | None = None):
        self._state = _Native()
        if _input is not None:
            self.extend(_input)

    def append(self, value: float):
        self._state.append(float(value))
        return self

    def extend(self, values: Any):
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
