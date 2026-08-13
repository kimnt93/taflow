# Python API

TAFlow's Python classes are thin adapters over persistent Rust states. Python
normalizes supported containers once; Rust owns validation, warm-up, rolling
state, arithmetic, output history, and scalar continuation.

## Install

Python 3.9 or newer and a Rust toolchain are required while prebuilt wheels are
not yet published:

```bash
pip install "git+https://github.com/kimnt93/taflow"
```

From a checkout, `make install` builds a release wheel, while `make dev`
installs an editable debug build. NumPy is required. Polars and Arrow adapters
are optional:

```bash
pip install "taflow[adapters]"
```

## Indicator lifecycle

Every lifecycle indicator uses the same sequence:

1. Construct an empty state using configuration only.
2. Ingest one or more chronological histories with `extend`.
3. Continue with `append` as new bars arrive.
4. Read the latest output through `value` or aligned history through `compute`.
5. Call `reset` to restore fresh-state behavior with the same configuration.

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=20)
assert len(ema) == 0

ema.extend(history)
ema.append(next_close)

latest = ema.value
aligned = ema.compute()
ema.reset()
```

`append`, `extend`, and `reset` mutate and return the same adapter, so fluent
calls are supported:

```python
values = ExponentialMovingAverage(timeperiod=20).extend(close).compute()
```

Historical input is deliberately invalid in a constructor. Use
`ExponentialMovingAverage(timeperiod=20)`, not
`ExponentialMovingAverage(close, timeperiod=20)`.

## Multiple inputs and outputs

`extend` accepts series in the same order that `append` accepts scalar values.
Input lengths are checked before native state is mutated:

```python
from taflow import AverageTrueRange, Aroon

atr = AverageTrueRange(timeperiod=14).extend(high, low, close)
atr.append(float(high[-1]), float(low[-1]), float(close[-1]))

down, up = Aroon(timeperiod=14).extend(high, low).compute()
```

Single-output `compute` returns a NumPy array. Multi-output indicators return a
documented tuple of arrays. `value` returns the matching scalar or tuple and is
`None` during warm-up.

## Warm-up and history

Histories stay aligned with input bars. Undefined warm-up positions are
`numpy.nan`; the scalar `value` property is `None` until the state is warm.
`append` remains fluent and returns the adapter. `compute` reads the native
output cache and does not replay earlier bars.

Bulk ingestion, repeated chunks, and scalar replay produce bitwise-identical
histories and leave identical continuation state. After `reset`, replaying the
same data reproduces the original result.

## Accepted data

NumPy arrays, Python lists, pandas Series, Polars Series, and Arrow arrays are
accepted. Data is converted to contiguous `float64` once per `extend` call.

```python
import numpy as np
import pandas as pd
from taflow import SimpleMovingAverage

close_array = np.asarray(close, dtype=np.float64)
close_series = pd.Series(close_array)

from_array = SimpleMovingAverage(timeperiod=10).extend(close_array).compute()
from_series = SimpleMovingAverage(timeperiod=10).extend(close_series).compute()
```

See [Data input/output](DATA.md) for dataframes, output converters, Polars,
Arrow, and custom adapters.

## Indicator pipeline

`TAPipeline` builds a causal graph from source columns, configured indicators,
expressions, and named outputs. Indicator instances passed into the graph must
be fresh configured states:

```python
from taflow import AverageTrueRange, ExponentialMovingAverage
from taflow.op import TAPipeline

pipe = TAPipeline()
high_source = pipe.source("high")
low_source = pipe.source("low")
close_source = pipe.source("close")

fast = pipe.indicator(
    "fast", ExponentialMovingAverage(timeperiod=12), close_source
)
slow = pipe.indicator(
    "slow", ExponentialMovingAverage(timeperiod=26), close_source
)
atr = pipe.indicator(
    "atr", AverageTrueRange(timeperiod=14),
    high_source, low_source, close_source,
)

pipe.output("spread", pipe.expression("spread", fast - slow))
pipe.output("atr", atr)
result = pipe.extend({"high": high, "low": low, "close": close})
```

See [Pipelines](PIPELINES.md) for evaluation rules, live `append`, reset, and
the current constraints around chained warm-up and multi-output nodes.

The generated [indicator class reference](INDICATORS.md) lists every full
class name with its complete constructor defaults, ordered `extend` inputs,
and corresponding Rust constructor parameters.

## Metrics

Metrics also separate configuration from input selection. Select the semantic
input domain with a `from_*` method, then compute or continue the same state:

```python
import numpy as np
from taflow.metrics import SharpeRatio

returns = np.array([0.01, -0.004, 0.006, 0.002])
sharpe = SharpeRatio(
    periods_per_year=252.0,
    annual_risk_free_rate=0.03,
).from_returns(returns)

sharpe.append(0.003)
value = sharpe.compute()
```

`MetricPipeline` fans one normalized input stream into several configured
metrics. See [Metric pipeline](METRIC_PIPELINE.md) for supported domains and
the complete lifecycle.

The generated [metric class reference](METRICS.md) is sorted by full class
name and lists all 57 Python constructors with default values, every supported
semantic input signature, the corresponding Rust signatures, outputs, minimum
observations, and definitions.

## Finding a class

The generated [indicator class reference](INDICATORS.md) and
[metric class reference](METRICS.md) list the canonical full class names in
alphabetical order. Both references include complete Python parameters and
defaults alongside their explicit Rust constructor parameters. Indicator
classes are exported from both `taflow` and `taflow.indicators`; metric classes
are exported from `taflow.metrics`.

## Verification

```bash
make check
uv run python scripts/verification/interfaces.py
make bench ARGS="EMA ATR"
```

Current generated evidence is available in [correctness](../verify/CORRECTNESS.md)
and [benchmark](../verify/BENCHMARK.md) reports.
