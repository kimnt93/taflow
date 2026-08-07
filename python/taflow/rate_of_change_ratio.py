"""Canonical Rate of Change Ratio adapter."""

from ._native import StatefulRocr
from ._unary_state import UnaryStateAdapter


class RateOfChangeRatio(UnaryStateAdapter):
    """Compute the rate-of-change ratio through Rust."""

    _native_cls = StatefulRocr
