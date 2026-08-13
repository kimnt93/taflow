"""Python adapter for the native Arms Index (TRIN)."""

from typing import Any
import numpy as np
from .._native import ArmsIndex as _Native
from .._series import as_float64_series


class ArmsIndex:
    """Compare the advance/decline ratio with the up/down volume ratio.

    The formula is ``(advancers / decliners) / (advancing_volume /
    declining_volume)`` with Wickra-compatible denominator floors. Inputs are
    pre-aggregated market breadth totals. This maps to Wickra ``Trin``.

    Args:
        advancers: Number of advancing issues at each tick.
        decliners: Number of declining issues at each tick.
        advancing_volume: Aggregate volume in advancing issues.
        declining_volume: Aggregate volume in declining issues.

    Raises:
        ValueError: If the four histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(
        self,
        advancers: float,
        decliners: float,
        advancing_volume: float,
        declining_volume: float,
    ) -> "ArmsIndex":
        """Append one aggregate breadth tick and return this adapter."""
        self._state.append(
            float(advancers),
            float(decliners),
            float(advancing_volume),
            float(declining_volume),
        )
        return self

    def extend(
        self, advancers: Any, decliners: Any, advancing_volume: Any, declining_volume: Any
    ) -> "ArmsIndex":
        """Append aligned aggregate histories after validating their lengths."""
        arrays = tuple(
            as_float64_series(item)
            for item in (advancers, decliners, advancing_volume, declining_volume)
        )
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("Arms Index inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest TRIN value, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned Arms Index history."""
        return self._state.compute()

    def reset(self) -> "ArmsIndex":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
