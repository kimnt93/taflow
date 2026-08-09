"""Persistent pointwise signed power."""

from typing import Any

import numpy as np

from ._native import SignedPowerOperator as _Native
from ._series import as_float64_series


class SignedPower:
    """Compute ``sign(x) * abs(x) ** exponent`` for an aligned series."""

    def __init__(
        self,
        _input: Any,
        exponent: float = 2.0,
    ) -> None:
        self._state = _Native(float(exponent))
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "SignedPower":
        """Append one scalar observation to the persistent native state."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "SignedPower":
        """Append an aligned input series to the persistent native state."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned output history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest computed value."""
        return self._state.value

    def reset(self) -> "SignedPower":
        """Clear output history while retaining the configured exponent."""
        self._state.reset()
        return self
