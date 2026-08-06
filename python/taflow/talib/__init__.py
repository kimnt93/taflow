"""TA-Lib-compatible uppercase batch functions.

This compatibility namespace delegates to the verified native ``taflow``
batch API and intentionally contains no numerical implementations.
"""

from taflow._native import *  # noqa: F401,F403
from taflow._native import get_function_groups, get_functions

__version__ = "0.1.2"
__ta_version__ = "0.6.4"


class MA_Type:
    """Moving-average selector values compatible with TA-Lib."""

    SMA = 0
    EMA = 1
    WMA = 2
    DEMA = 3
    TEMA = 4
    TRIMA = 5
    KAMA = 6
    MAMA = 7
    T3 = 8
