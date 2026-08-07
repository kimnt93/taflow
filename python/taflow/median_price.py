"""Canonical median-price adapter."""

from ._native import StatefulMedprice
from ._price_state import HlPriceState


class MedianPrice(HlPriceState):
    """Compute median price through the native Rust kernel."""

    _native_cls = StatefulMedprice
