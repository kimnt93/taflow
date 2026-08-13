"""Python adapter for the native McClellan Oscillator."""

from typing import Any
import numpy as np
from .._native import McClellanOscillator as _Native
from .._series import as_float64_series


class McClellanOscillator:
    """Subtract slow from fast EMA of ratio-adjusted net advances.

    Ratio-adjusted net advances are ``1000 * (advancers - decliners) /
    max(advancers + decliners, 1)``. The native 19/39 EMA pair is seeded on the
    first tick, which emits zero. This maps to Wickra ``McClellanOscillator``.

    Args:
        advancers: Number of advancing issues at each tick.
        decliners: Number of declining issues at each aligned tick.

    Raises:
        ValueError: If the two histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, advancers: float, decliners: float) -> "McClellanOscillator":
        """Append one breadth tick and return this adapter."""
        self._state.append(float(advancers), float(decliners))
        return self

    def extend(self, advancers: Any, decliners: Any) -> "McClellanOscillator":
        """Append aligned breadth histories after length validation."""
        arrays = as_float64_series(advancers), as_float64_series(decliners)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("advancers and decliners must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned oscillator history."""
        return self._state.compute()

    def reset(self) -> "McClellanOscillator":
        """Reset both EMA states and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
