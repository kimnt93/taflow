from typing import Any

import numpy as np

from .._native import BatPattern as _Native
from .._series import as_float64_series


class BatPattern:
    """Causal Bat harmonic reversal detector over aligned OHLC bars.

    Rust owns the bounded persistent window, pattern arithmetic, warm-up, and
    output history. The required input order is open, high, low, close. Scalar
    output is 1.0 for a bullish completion, -1.0 for a bearish completion, and
    0.0 otherwise, including before five confirmed alternating pivots exist.
    The detector applies Wickra's XA, AB, BC, CD, and AD ratio intervals when
    a new pivot is confirmed; signals therefore remain causal and do not repaint.
    append, extend, and reset mutate and return this adapter. The external name
    mapping is Wickra 0.9.9 Bat; TA-Lib has no equivalent pattern function.
    """

    def __init__(self, open: Any, high: Any, low: Any, close: Any) -> None:
        """Initialize and process required aligned chronological OHLC series.

        Empty aligned series create a fresh streaming state. Differing input
        lengths raise ValueError before native state is mutated.
        """
        self._state = _Native()
        self.extend(open, high, low, close)

    def append(
        self, open: float, high: float, low: float, close: float
    ) -> "BatPattern":
        """Append one open/high/low/close bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "BatPattern":
        """Append aligned OHLC histories and return this adapter."""
        series = tuple(as_float64_series(item) for item in (open, high, low, close))
        if len({len(item) for item in series}) != 1:
            raise ValueError("OHLC inputs must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest native signal, or None during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native signal history as a NumPy array."""
        return self._state.compute()

    def reset(self) -> "BatPattern":
        """Restore fresh native state without reallocating and return self."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
