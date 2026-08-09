"""Native-backed McGinley Dynamic adapter."""

from typing import Any

import numpy as np

from ._native import McGinleyDynamicOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class McGinleyDynamic:
    """Compute the causal McGinley Dynamic moving average.

    ``close`` is the required chronological price history and may be empty
    for a fresh stream. ``length`` defaults to 10 and ``c`` defaults to 1.0;
    both are validated by Rust. Rust owns the recurrence, warm-up, and aligned
    output. ``compute`` returns one float array, ``value`` is ``None`` before
    the first bar, and lifecycle mutators return ``self``. The independent
    oracle/name mapping is pandas-ta-classic ``mcginley``.
    """

    def __init__(self, close: Any, length: int = 10, c: float = 1.0) -> None:
        """Initialize and process the supplied close history.

        Parameters
        ----------
        close : object
            Required chronological close prices; empty creates a fresh state.
        length : int, default 10
            Positive recurrence length.
        c : float, default 1.0
            Positive McGinley adjustment constant.
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
        """Return the latest average, or ``None`` while empty."""
        return self._state.value

    def reset(self) -> "McGinleyDynamic":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of close prices processed by Rust."""
        return adapter_length(self)


__all__ = ["McGinleyDynamic"]
