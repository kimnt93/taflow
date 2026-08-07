"""TA-Lib-compatible uppercase batch functions.

This compatibility namespace delegates to the verified native ``taflow``
batch API and intentionally contains no numerical implementations.
"""

from enum import IntEnum

from taflow._native import *  # noqa: F401,F403
from taflow._native import get_function_groups as _native_get_function_groups
from taflow._native import get_functions as _native_get_functions


def get_functions() -> list[str]:
    """Return the upstream TA-Lib function registry with canonical aliases."""
    names = list(_native_get_functions())
    if "TripleExponentialAverage" in names:
        names.remove("TripleExponentialAverage")
    if "T3" not in names:
        names.append("T3")
    return names


def get_function_groups() -> dict[str, list[str]]:
    """Return grouped registry metadata using the TA-Lib spelling ``T3``."""
    groups = _native_get_function_groups()
    for names in groups.values():
        if "TripleExponentialAverage" in names:
            names[names.index("TripleExponentialAverage")] = "T3"
    return groups


# Older extension wheels expose the descriptive native name; keep the public
# compatibility surface stable while newer wheels may provide a direct T3.
try:
    T3 = getattr(__import__("taflow._native", fromlist=["T3"]), "T3")
except AttributeError:
    T3 = __import__("taflow._native", fromlist=["TripleExponentialAverage"]).TripleExponentialAverage

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
