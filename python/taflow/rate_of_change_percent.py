"""Canonical fractional Rate of Change adapter."""

from ._native import StatefulRocp
from ._unary_state import UnaryStateAdapter


class RateOfChangePercent(UnaryStateAdapter):
    """Compute fractional rate of change through Rust."""

    _native_cls = StatefulRocp
