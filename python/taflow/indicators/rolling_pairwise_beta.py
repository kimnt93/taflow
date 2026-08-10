"""Public adapter for native rolling pairwise beta."""

from typing import Any

import numpy as np

from .._native import RollingPairwiseBeta as _Native
from .._series import as_float64_series


class RollingPairwiseBeta:
    """Regress one asset's log returns on another asset's log returns.

    Inputs are positive raw prices. One price seeds returns and ``period``
    return pairs seed rolling OLS, so the first value is at index ``period``.
    A zero benchmark variance returns zero. This maps to Wickra
    ``PairwiseBeta``.

    Args:
        asset: Initial prices for the dependent asset.
        benchmark: Initial prices for the explanatory asset.
        period: Number of return pairs, default 20 and minimum 2.

    Raises:
        ValueError: If series lengths differ or ``period`` is below two.
    """

    def __init__(self, asset: Any, benchmark: Any, period: int = 20) -> None:
        """Initialize native state and process aligned price histories."""
        self._state = _Native(period)
        self.extend(asset, benchmark)

    def append(self, asset: float, benchmark: float) -> "RollingPairwiseBeta":
        """Append one aligned price pair and return this instance."""
        self._state.append(float(asset), float(benchmark))
        return self

    def extend(self, asset: Any, benchmark: Any) -> "RollingPairwiseBeta":
        """Append aligned price histories and return this instance."""
        asset_series = as_float64_series(asset)
        benchmark_series = as_float64_series(benchmark)
        if len(asset_series) != len(benchmark_series):
            raise ValueError("asset and benchmark must have equal lengths")
        self._state.extend(asset_series, benchmark_series)
        return self

    @property
    def value(self) -> float | None:
        """Return latest beta, or ``None`` until enough returns exist."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned beta history with warm-up ``NaN`` values."""
        return self._state.compute()

    def reset(self) -> "RollingPairwiseBeta":
        """Clear prices and regression state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of price pairs stored by native state."""
        return len(self._state)
