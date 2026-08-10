"""Native-backed McGinley Dynamic adapter."""

from typing import Any

import numpy as np

from .._native import McGinleyDynamicOperator as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class McGinleyDynamic:
    """Compute the causal McGinley Dynamic moving average.

    The state is seeded by the simple mean of the first ``length`` closes.
    Later bars use ``previous + (close - previous) / (c * length *
    (close / previous) ** 4)``. ``c`` defaults to John McGinley's constant
    ``0.6``. Rust owns arithmetic and warm-up; aligned history contains NaN
    before the seed is complete. The independent oracle is Wickra
    ``McGinleyDynamic``. Lifecycle mutators return this adapter.
    """

    def __init__(self, close: Any, length: int = 10, c: float = 0.6) -> None:
        """Initialize and process the supplied close history.

        Parameters
        ----------
        close : object
            Required chronological close prices; empty creates a fresh state.
        length : int, default 10
            Positive recurrence length.
        c : float, default 0.6
            Positive adjustment constant; ``0.6`` matches Wickra.
        """
        self._state = _Native(int(length), float(c))
        self.extend(close)

    def append(self, close: float) -> "McGinleyDynamic":
        """Append one close price and return this adapter."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "McGinleyDynamic":
        """Append a chronological close history and return this adapter."""
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned McGinley Dynamic history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest average, or ``None`` during seed warm-up."""
        return self._state.value

    def reset(self) -> "McGinleyDynamic":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of close prices processed by Rust."""
        return adapter_length(self)


__all__ = ["McGinleyDynamic"]
