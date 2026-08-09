"""Persistent cumulative observation count."""

from typing import Any

import numpy as np

from ._native import CumulativeCountOperator as _Native
from ._series import as_float64_series


class CumulativeCount:
    """Emit the one-based number of observations processed at each bar."""

    def __init__(
        self,
        _input: Any,
    ) -> None:
        self._state = _Native()
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "CumulativeCount":
        """Count one scalar observation."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "CumulativeCount":
        """Count an aligned input series."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned cumulative-count history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest one-based count."""
        return self._state.value

    def reset(self) -> "CumulativeCount":
        """Reset the count and clear output history."""
        self._state.reset()
        return self
