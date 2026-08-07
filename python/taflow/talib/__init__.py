"""TA-Lib-compatible uppercase batch functions.

This compatibility namespace delegates to the verified native ``taflow``
batch API and intentionally contains no numerical implementations.
"""

from enum import IntEnum

from taflow._native import *  # noqa: F401,F403
from taflow._native import get_function_groups, get_functions

__version__ = "0.1.2"
__ta_version__ = "0.6.4"


class MaType(IntEnum):
    """Moving-average selector used by canonical APIs.

    Parameters
    ----------
    value : int
        TA-Lib selector value for the moving-average algorithm.

    Returns
    -------
    MaType
        Integer-compatible selector accepted by Python and Rust.
    """

    SMA = 0
    EMA = 1
    WMA = 2
    DEMA = 3
    TEMA = 4
    TRIMA = 5
    KAMA = 6
    MAMA = 7
    T3 = 8


# Preserve TA-Lib's historical spelling on the compatibility surface.
MA_Type = MaType


__all__ = ["MaType", "MA_Type", *get_functions()]
