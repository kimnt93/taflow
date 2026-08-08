# Pipelines

A pipeline is a **causal dependency graph** over indicators and expressions.
You describe what you want computed; the graph dispatches each bar through
every node exactly once, keeps stateful nodes alive between bars, and returns
aligned outputs.

Two properties make it worth using over calling indicators directly:

1. **Each node is evaluated once per bar**, no matter how many outputs depend
   on it. Two outputs sharing an EMA cost one EMA update, not two.
2. **It streams.** The same graph serves a historical backfill (`extend`) and
   a live feed (`append`), with identical results.

Everything here is covered by [`tests/test_pipeline.py`](../tests/test_pipeline.py).

## Import

```python
from taflow.op import TAPipeline
```

`taflow.op` is the short alias for `taflow.executions`; both export the same
objects. Indicator classes are **not** in this namespace — import those from
`taflow`.

## Building a graph

Four calls build any graph.

| Call | Purpose |
|---|---|
| `pipe.source(field)` | Declares an input column. Memoized: calling it twice with the same name returns the same node. |
| `pipe.indicator(name, state, *inputs)` | A stateful node. `state` is any object with `append(*values)` — every TAFlow indicator qualifies. |
| `pipe.expression(name, expr)` | A derived node built from arithmetic on other nodes. |
| `pipe.output(name, node)` | Exposes a node under a result name. |

```python
from taflow.op import TAPipeline
from taflow import ExponentialMovingAverage, AverageTrueRange

pipe = TAPipeline()

# Node handles are named apart from the data arrays so the two never collide.
high_s, low_s, close_s = pipe.source("high"), pipe.source("low"), pipe.source("close")

fast = pipe.indicator("fast", ExponentialMovingAverage(timeperiod=12), close_s)
slow = pipe.indicator("slow", ExponentialMovingAverage(timeperiod=26), close_s)
atr  = pipe.indicator("atr",  AverageTrueRange(timeperiod=14), high_s, low_s, close_s)

pipe.output("macd", pipe.expression("macd", fast - slow))
pipe.output("normalized", pipe.expression("normalized", (fast - slow) / atr))
pipe.output("atr", atr)
```

`fast` and `slow` each feed two outputs, and `atr` feeds two — but every bar
steps each of them exactly once.

### Expressions

Nodes support `+`, `-`, `*`, `/`, unary `-`, and the reflected forms, against
other nodes or plain numbers:

```python
spread   = fast - slow
upper    = fast + 2.0 * atr
inverted = -spread
ratio    = 100.0 / atr          # scalar on the left works too
```

Expressions are lazy: they are evaluated per bar during dispatch, in
dependency order, from already-computed values. They hold no state.

An indicator can consume another indicator's output directly:

```python
ema      = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=5), close_s)
smoothed = pipe.indicator("smoothed", SimpleMovingAverage(timeperiod=7), ema)
```

> **Chaining currently propagates warm-up NaN, and that is destructive.**
> An upstream indicator emits `NaN` during its warm-up. Those `NaN`s are fed
> into the downstream indicator, and a sum-based state (SMA, VAR, STDDEV,
> anything keeping a running total) never recovers: the accumulator does
> `sum += new - old`, so once it is `NaN` it stays `NaN` for the rest of the
> stream. `SimpleMovingAverage(timeperiod=7)` fed a 5-period EMA over 200 bars
> returns **200 NaNs**, not 11.
>
> Until this is addressed, chain only onto states that tolerate `NaN` (the
> EMA-family recurrences recover; windowed sums do not), or compute the
> upstream separately, strip its warm-up, and feed the warm values in.

## Running a graph

### Whole columns

```python
result = pipe.extend({"high": high, "low": low, "close": close})
result["macd"]          # np.ndarray, same length as the input
```

Every registered output comes back as a `float64` array the same length as the
input, `NaN` through warm-up. All input columns must be the same length — a
mismatch raises `ValueError`.

### One bar at a time

```python
tick = pipe.append({"high": 101.2, "low": 99.8, "close": 100.5})
tick["macd"]            # float for this bar
```

`append` takes a mapping with a value for every source field and returns a
dict of the current output values. This is the live path: cost per bar is
constant regardless of how much history the graph has seen.

`extend` is exactly a loop of `append` over the columns — verified bitwise —
so you can backfill with `extend` and then continue with `append` and get the
same numbers you would have had feeding every bar individually.

### Inspecting and resetting

```python
pipe.outputs        # tuple of registered output names
pipe.reset()        # reset every stateful node reachable from the outputs
```

After `reset()`, replaying the same data reproduces the first run exactly.

## Evaluate-once: how it works, and why it matters

A stateful node stepped twice in one bar would advance its indicator twice and
silently corrupt every subsequent value. The graph prevents this with a
per-bar memo table keyed by node identity:

- `Pipeline.append` creates one cache for the bar, then evaluates each output
  through `_evaluate`.
- `_evaluate` returns the cached value if the node was already computed this
  bar; otherwise it computes it and stores it.
- Arithmetic nodes carry their operands explicitly (`_operation`) so the
  evaluator recurses through the same cache rather than calling the operand
  closures directly — without this, `fast - slow` would step both EMAs a
  second time.

This is asserted with a counting spy in `tests/test_pipeline.py`, not just
assumed: one node feeding four outputs (including an expression that
references it twice) receives exactly one `append` per bar, and chained
indicators each receive exactly one.

## Gotcha: unreachable nodes are never stepped

The graph is **demand-driven from the registered outputs**. An indicator you
create but never wire to an output — directly or as a dependency of one — is
never stepped and stays at bar zero:

```python
used   = pipe.indicator("used", SimpleMovingAverage(timeperiod=5), close_s)
unused = pipe.indicator("unused", SimpleMovingAverage(timeperiod=5), close_s)
pipe.output("used", used)

pipe.extend({"high": high, "low": low, "close": close})
# `used` has consumed every bar; `unused` has consumed none.
```

If a node's state has to track the feed — because you read `state.value`
yourself, or you plan to attach it as an output later — register it as an
output. Adding an output you ignore costs one array in `extend` and nothing in
`append`.

## When a pipeline is the right tool

**Use one when** several outputs share sub-computations, when you want one
object to drive from a live feed, or when you want indicator outputs combined
arithmetically without materializing intermediates.

**Skip it when** you need a single indicator over a single array — the class
API is more direct and its bulk path is a compiled Rust kernel, whereas the
pipeline dispatches per bar in Python. A pipeline's advantage is *sharing* and
*streaming*, not raw bulk throughput: for one indicator over one array,
`SimpleMovingAverage(close, timeperiod=30).compute()` is faster.

## Custom nodes

`pipe.indicator` accepts anything exposing `append(*values)`, optionally
`value` and `reset()`. That covers every TAFlow indicator, and lets you drop
in your own state:

```python
class RunningPeak:
    def __init__(self):
        self.value = float("-inf")

    def append(self, x):
        self.value = max(self.value, x)
        return self

    def reset(self):
        self.value = float("-inf")

peak = pipe.indicator("peak", RunningPeak(), close_s)
```

If `append` returns the state object itself (the fluent style TAFlow
indicators use), the graph reads the scalar from `.value`; if it returns a
number, that number is used directly. `None` becomes `NaN`.

## Related

- [Indicator reference](INDICATORS.md) — all 299 classes, parameters, and
  constructor order.
- Converters (`AdaptInput`, `ToNumpy`, `ToPandas`, `ToPolars`, `ToArrow`,
  `ToList`) live in the same `taflow.op` namespace; see the README.
