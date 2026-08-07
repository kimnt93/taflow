"""Canonical Williams %R adapter."""

from ._native import StatefulWillr
from ._ohlc_state import OhlcStateAdapter


class WilliamsR(OhlcStateAdapter):
    """Compute Williams %R from aligned high, low, and close prices."""

    _native_cls = StatefulWillr
