"""Canonical typical-price adapter."""

from ._native import StatefulTypprice
from ._price_state import HlcPriceState


class TypicalPrice(HlcPriceState):
    """Compute typical price through the native Rust kernel."""

    _native_cls = StatefulTypprice
