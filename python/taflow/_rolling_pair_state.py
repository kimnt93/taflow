"""Shared native boundary for two-output unary rolling states."""

from ._unary_state import UnaryStateAdapter


class RollingPairState(UnaryStateAdapter):
    def __init__(self, timeperiod: int = 30) -> None:
        self._state = self._native_cls(timeperiod)
