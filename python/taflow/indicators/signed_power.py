"""Canonical native-backed signed-power adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import SignedPowerOperator as _Native
from .._series import as_float64_series


class SignedPower:
    """Pointwise sign(x) times abs(x) raised to exponent."""

    def __init__(self, exponent: float = 2.0) -> None:
        self._state = _Native(float(exponent))

    def append(self, input: float) -> "SignedPower":
        self._state.append(float(input))
        return self

    def extend(self, input: Any) -> "SignedPower":
        values = as_float64_series(input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "SignedPower":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
