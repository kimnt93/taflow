"""Canonical native-backed CUSUM control-chart adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import CumulativeSumControlChartOperator as _Native
from .._series import as_float64_series


class CumulativeSumControlChart:
    """Cumulative-sum event flags for a required chronological change series.
    threshold is the non-negative deviation threshold. Scalar output is None
    only before the first observation.
    """

    def __init__(self, change: Any, threshold: float = 1.0) -> None:
        self._state = _Native(float(threshold))
        self.extend(change)

    def append(self, change: float) -> "CumulativeSumControlChart":
        self._state.append(float(change))
        return self

    def extend(self, change: Any) -> "CumulativeSumControlChart":
        values = as_float64_series(change)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "CumulativeSumControlChart":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
