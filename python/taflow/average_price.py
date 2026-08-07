"""Canonical average-price adapter."""

from ._native import StatefulAvgprice
from ._price_state import OhlcPriceState


class AveragePrice(OhlcPriceState):
    """Compute average price through the native Rust kernel."""

    _native_cls = StatefulAvgprice
