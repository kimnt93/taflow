"""Canonical True Range adapter."""

from ._native import StatefulTrange
from ._ohlc_state import OhlcStateAdapter


class TrueRange(OhlcStateAdapter):
    """Compute true range from high, low, and previous close values."""

    _native_cls = StatefulTrange
    _period_required = False
