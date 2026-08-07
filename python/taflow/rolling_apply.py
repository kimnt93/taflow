"""Causal Python rolling application helper.

This operator is intentionally batch-only because an arbitrary callable has no
general incremental state representation.
"""
from collections.abc import Callable
from typing import Any

import numpy as np

from ._series import as_float64_series


def rolling_apply(_input: Any, timeperiod: int, function: Callable[[np.ndarray], float]) -> np.ndarray:
    """Apply ``function`` to each full trailing window.

    The callable receives a read-only NumPy view of each causal window. The
    first ``timeperiod - 1`` outputs are ``NaN``.
    """
    if timeperiod < 1:
        raise ValueError("timeperiod must be >= 1")
    if not callable(function):
        raise TypeError("function must be callable")
    values = as_float64_series(_input)
    output = np.full(values.shape, np.nan, dtype=np.float64)
    for index in range(timeperiod - 1, len(values)):
        window = values[index + 1 - timeperiod:index + 1]
        window.setflags(write=False)
        output[index] = function(window)
    return output
