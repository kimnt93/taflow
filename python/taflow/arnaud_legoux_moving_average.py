"""Native-backed Arnaud Legoux moving-average adapter."""

from typing import Any

import numpy as np

from ._native import AlmaOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class ArnaudLegouxMovingAverage:
    """Compute the Gaussian-weighted Arnaud Legoux moving average.

    ``_input`` is the required chronological price series and may be empty for
    a fresh stream. ``timeperiod`` defaults to 10, ``offset`` to 0.85, and
    ``sigma`` to 6.0. Rust owns Gaussian weights, warm-up, and aligned output;
    ``compute`` returns one float array, ``value`` is the latest scalar or
    ``None`` during warm-up, and lifecycle mutators return ``self``. The oracle
    is pandas-ta-classic ``alma``; its initialization difference is recorded as
    a documented VARIANT by the verifier.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 10,
        offset: float = 0.85,
        sigma: float = 6.0,
    ) -> None:
        self._state = _Native(int(timeperiod), float(offset), float(sigma))
        self.extend(_input)

    def append(self, _input: float) -> "ArnaudLegouxMovingAverage":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "ArnaudLegouxMovingAverage":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Arnaud Legoux average history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest average, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "ArnaudLegouxMovingAverage":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return adapter_length(self)


__all__ = ["ArnaudLegouxMovingAverage"]
