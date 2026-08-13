from typing import Any
import numpy as np
from .._native import InverseFisherTransform as _Native
from .._series import as_float64_series


class InverseFisherTransform:
    """Scaled hyperbolic-tangent inverse Fisher transform."""

    def __init__(self, scale: float = 1.0) -> None:
        self._state = _Native(scale)

    def append(self, value: float) -> "InverseFisherTransform":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "InverseFisherTransform":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "InverseFisherTransform":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
