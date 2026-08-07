"""Canonical weighted-close adapter."""

from ._native import StatefulWclprice
from ._price_state import HlcPriceState


class WeightedClose(HlcPriceState):
    """Compute weighted close through the native Rust kernel."""

    _native_cls = StatefulWclprice
