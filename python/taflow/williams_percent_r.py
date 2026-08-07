"""Canonical Williams %R adapter."""

from ._native import StatefulWillr
from ._ohlc_state import OhlcStateAdapter


class WilliamsPercentR(OhlcStateAdapter):
    """Compute Williams Percent R from aligned high, low, and close prices.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned price histories. Later bars are supplied through
        ``append`` or ``extend``.
    timeperiod : int, optional
        Trailing lookback used by the native kernel.

    Returns
    -------
    WilliamsPercentR
        A persistent native-backed indicator state.
    """

    _native_cls = StatefulWillr
