"""Shared conversion and alignment validation for OHLC adapters."""

from typing import Any

import numpy as np

from ._series import as_float64_series


def as_ohlc_arrays(
    _open: Any, high: Any, low: Any, close: Any
) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
    """Convert four OHLC inputs once and reject misaligned histories."""
    arrays = tuple(
        as_float64_series(series) for series in (_open, high, low, close)
    )
    if any(array.shape != arrays[0].shape for array in arrays[1:]):
        raise ValueError("open, high, low, and close must have equal lengths")
    return arrays  # type: ignore[return-value]
