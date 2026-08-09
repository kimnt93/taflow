"""Shared native boundary for two-output unary rolling states."""

from typing import Any

from ._unary_state import UnaryStateAdapter


class RollingPairState(UnaryStateAdapter):
    def __init__(self, _input: Any, timeperiod: int = 30) -> None:
        self._state = self._native_cls(timeperiod)
        self.extend(_input)
