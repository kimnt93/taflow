"""Native-backed rolling cointegration diagnostic."""
from typing import Any
import numpy as np
from .._native import RollingCointegration as _Native
from .._series import as_float64_series

class RollingCointegration:
    """Run a rolling Engle-Granger cointegration diagnostic.

    Rust fits ``left = intercept + hedge_ratio * right`` and runs a no-constant
    augmented Dickey-Fuller regression on the residual spread. Output order is
    ``(hedge_ratio, spread, augmented_dickey_fuller_statistic)`` and maps to
    Wickra ``Cointegration``. More-negative ADF values indicate stronger mean
    reversion; warm-up rows are ``NaN``.

    Args:
        left: Required dependent price-level history.
        right: Required predictor price-level history.
        period: Rolling regression window. Defaults to 30.
        augmented_dickey_fuller_lags: Lagged spread differences. Defaults to 1.

    Raises:
        ValueError: If histories differ in length or the period is too short.
    """
    def __init__(self, left: Any, right: Any, period: int = 30, augmented_dickey_fuller_lags: int = 1) -> None: self._state = _Native(period, augmented_dickey_fuller_lags); self.extend(left, right)
    def append(self, left: float, right: float) -> "RollingCointegration": self._state.append(float(left), float(right)); return self
    def extend(self, left: Any, right: Any) -> "RollingCointegration":
        x, y = as_float64_series(left), as_float64_series(right)
        if len(x) != len(y): raise ValueError("left and right inputs must have equal lengths")
        self._state.extend(x, y); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float] | None: return self._state.value
    def reset(self) -> "RollingCointegration": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
