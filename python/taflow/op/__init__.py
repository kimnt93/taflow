"""Short execution namespace for TAFlow.

`taflow.op` is an ergonomic alias of :mod:`taflow.executions`. Indicator and
extended-series classes do not belong here; import those from :mod:`taflow`
or their dedicated modules.
"""

from ..executions import (
    TAAdapterGateway,
    TAArrowAdapter,
    TAExpr,
    TANumpyAdapter,
    TAPipeline,
    TAPipelines,
    TAPolarsAdapter,
    TAPythonListAdapter,
    AdaptInput,
    AdaptOutput,
    RollingApply,
    SessionFlags,
    ToArrow,
    ToList,
    ToNumpy,
    ToPandas,
    ToPolars,
)

__all__ = [
    "TAPipeline", "TAPipelines", "TAExpr", "TAAdapterGateway",
    "TANumpyAdapter", "TAPythonListAdapter", "TAArrowAdapter",
    "TAPolarsAdapter", "AdaptInput", "AdaptOutput", "RollingApply",
    "SessionFlags", "ToNumpy", "ToList", "ToArrow", "ToPolars", "ToPandas",
]
