"""Canonical Rate of Change adapter."""

from ._native import StatefulRoc
from ._unary_state import UnaryStateAdapter


class RateOfChange(UnaryStateAdapter):
    """Compute percentage rate of change through Rust."""

    _native_cls = StatefulRoc
