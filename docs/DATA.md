# Data in and out

TAFlow computes on contiguous `float64`. You rarely have to think about that:
pass whatever container you already have, and it is converted once at the
boundary. This page covers every accepted input form, every output form, and
how to plug in a container TAFlow does not know about.

All examples on this page are executed as part of the docs check.

## Input: pass what you have

Every indicator accepts NumPy arrays, Python lists, pandas Series, Polars
Series, and Arrow arrays interchangeably:

```python
import numpy as np, pandas as pd, polars as pl, pyarrow as pa
from taflow import SimpleMovingAverage

close = np.cumsum(np.random.default_rng(0).normal(0, 1, 500)) + 100.0

SimpleMovingAverage(timeperiod=10).extend(close).compute()                    # numpy
SimpleMovingAverage(timeperiod=10).extend(close.tolist()).compute()           # list
SimpleMovingAverage(timeperiod=10).extend(pd.Series(close)).compute()         # pandas
SimpleMovingAverage(timeperiod=10).extend(pl.Series("close", close)).compute()# polars
SimpleMovingAverage(timeperiod=10).extend(pa.array(close)).compute()          # arrow
```

All five produce the identical `float64` NumPy array. Conversion happens once
per call — after that every bar is processed in Rust.

Polars and Arrow support come from optional extras:

```bash
pip install "taflow[adapters]"     # pyarrow + polars
```

### Dataframes

A single-column dataframe is unambiguous and works directly. With more than
one column you must say which:

```python
from taflow.op import AdaptInput

frame = pd.DataFrame({"open": o, "high": h, "low": l, "close": c,
                      "volume": v})

SimpleMovingAverage(timeperiod=10).extend(AdaptInput(frame, column="close")).compute()
```

Passing a multi-column frame without `column=` raises
`ValueError: column is required when a dataframe has multiple columns` rather
than silently guessing.

In practice you will usually just hand over the columns, which needs no
adapter at all:

```python
from taflow import MoneyFlowIndex

mfi = MoneyFlowIndex(timeperiod=14).extend(
    frame["high"], frame["low"], frame["close"], frame["volume"]
).compute()
```

The same line works unchanged if `frame` is a Polars DataFrame.

## Output: convert on the way out

`compute()` always returns NumPy. The `To*` helpers convert to whatever you
want to continue in:

```python
from taflow.op import ToNumpy, ToList, ToPandas, ToPolars, ToArrow

sma = SimpleMovingAverage(timeperiod=10).extend(close).compute()

ToNumpy(sma)                      # np.ndarray (identity, for symmetry)
ToList(sma)                       # list[float]
ToPandas(sma, name="sma_10")      # pandas Series
ToPolars(sma, name="sma_10")      # polars Series
ToArrow(sma)                      # pyarrow array
```

Attaching results back to a frame is the common case:

```python
frame["sma_10"] = SimpleMovingAverage(timeperiod=10).extend(frame["close"]).compute()
```

Multi-output indicators return a tuple, so unpack before converting:

```python
from taflow import BollingerBands

upper, middle, lower = BollingerBands(period=20).extend(close).compute()
frame["bb_upper"] = upper
```

## The generic gateway

`AdaptInput` and `AdaptOutput` are the general forms the `To*` helpers wrap:

```python
from taflow.op import AdaptInput, AdaptOutput

values = AdaptInput(frame, adapter="numpy", column="close")
series = AdaptOutput(sma, adapter="polars", name="sma_10")
```

Registered adapter names are `numpy`, `list`, `arrow`, and `polars` — the
`numpy` adapter is the one that understands pandas and Polars frames on the way
in. Register
your own container type once and it becomes available to both:

```python
from taflow.op import TAAdapterGateway

class MyAdapter:
    @staticmethod
    def input(values, *, column=None):
        return np.asarray(values, dtype=np.float64)

    @staticmethod
    def output(values):
        return MyContainer(values)

TAAdapterGateway.register("mine", MyAdapter)
AdaptInput(my_container, adapter="mine")
```

An adapter needs an `input(values, *, column=None)` and an `output(values)`;
both may be `staticmethod` or `classmethod`.

## Two extra helpers

`RollingApply` runs a custom reducer over a rolling window:

```python
from taflow.op import RollingApply

result = RollingApply(close, 20, lambda w: float(w.max() - w.min()))
```

`SessionFlags` turns session identifiers (dates, session ids, anything that
changes at a boundary) into the native boundary flags that session-aware
indicators consume:

```python
from taflow.op import SessionFlags
from taflow import Sessions

flags = SessionFlags(frame["date"])
high, low = Sessions().extend(new_session=flags, high=frame["high"], low=frame["low"]).compute()
```

## Cost

Conversion is one pass over the data and one allocation, per call — not per
bar. It is invisible on a 100k-bar `extend` and only worth thinking about if
you call `extend` with very small chunks in a tight loop. In that case,
convert once yourself to a `float64` NumPy array and pass that.

`compute()` copies the cached history once into a fresh array, so treat it as
a snapshot: mutating the returned array does not affect the indicator.

## Related

- [Indicator reference](INDICATORS.md) — every class, its input order, and its
  constructor configuration.
- [Streaming](STREAMING.md) — live updates and the chunk-invariance contract.
- [Pipelines](PIPELINES.md) — computing many indicators in one pass.
