# Streaming

Streaming is what TAFlow is built around. Every indicator keeps bounded state
in Rust, so `append(bar)` costs the same on bar 10 and on bar 10,000,000 — no
window is recomputed, nothing is re-scanned. A batch library has to redo work
proportional to the window on every tick; TAFlow does not.

All examples on this page are executed as part of the docs check.

## The lifecycle

Every one of the 299 classes exposes the same six operations:

```python
from taflow import ExponentialMovingAverage

ema = ExponentialMovingAverage(timeperiod=20)

ema.extend(history)        # backfill a whole array
ema.append(next_close)     # O(1) update for one new bar

ema.value                  # latest value, or None during warm-up
ema.compute()              # the full aligned series, from cache
len(ema)                   # bars consumed so far
ema.reset()                # clear state and history, in place
```

`append` returns the indicator itself, so calls chain:
`ema.append(a).append(b)`.

Multi-series indicators take their bar values positionally in the same order
as their constructor:

```python
from taflow import AverageTrueRange

atr = AverageTrueRange(timeperiod=14)
atr.extend(high_history, low_history, close_history)
atr.append(high_tick, low_tick, close_tick)
```

## Warm-up

An indicator cannot produce a value until it has seen enough bars. During that
period `value` is `None` and `compute()` reports `NaN`, so the output series
always lines up index-for-index with the input:

```python
sma = SimpleMovingAverage(timeperiod=30)
sma.extend(close)                 # 500 bars


out = sma.compute()
len(out)                          # 500 — same length as the input
np.isnan(out[:29]).all()          # True  — warm-up
np.isnan(out[29])                 # False — first real value
```

Use `value is None` rather than `math.isnan` if you want to branch on warm-up
in a live loop; it avoids a NaN check per tick.

## Backfill, then go live

The normal production shape: load history once, then feed the feed.

```python
ema = ExponentialMovingAverage(timeperiod=20)
ema.extend(history)                       # thousands of bars, one call

for tick in live_feed():                  # then one bar at a time, forever
    ema.append(tick.close)
    if ema.value is not None and ema.value > tick.close:
        ...
```

This is safe because of **chunk invariance**: the state after `extend(history)`
is bit-for-bit what it would be after appending each of those bars
individually. Splitting the input differently never changes a result:

```python
whole = SimpleMovingAverage(timeperiod=14); whole.extend(close)

split = SimpleMovingAverage(timeperiod=14)
for start in range(0, len(close), 37):
    split.extend(close[start:start + 37])

(whole.compute() == split.compute()).all()     # True, bitwise
```

The verification harness asserts this for all 287 functions at chunk sizes 1,
10, and 1000 on every run, alongside a 9,000-bar backfill followed by 1,000
live `append` calls compared against the reference implementation.

## Reading history

`compute()` returns the whole aligned series from a Rust-side cache. It never
recomputes, so calling it in a loop is cheap — it is a single memcpy
(~1 ms per million bars):

```python
ema.append(float(close[-1]))
series = ema.compute()            # every value so far, including this bar
```

The returned array is a snapshot; mutating it does not affect the indicator.
If you only need the latest number, `value` avoids the copy entirely.

`reset()` clears state and cached history in place, without reallocating, so
you can reuse one object across symbols or sessions:

```python
for symbol, bars in feeds.items():
    ema.reset()
    ema.extend(bars)
    results[symbol] = ema.compute()
```

## Cost

Per-bar update is **O(1)** and allocation-free — no indicator allocates in
`append`. The measured cost per `append` through the Python boundary is
roughly 0.15–0.5 µs, and that figure is dominated by the Python call itself,
not the indicator: the Rust work behind it is single-digit nanoseconds for
most functions.

Two consequences worth knowing:

- **For a live feed, TAFlow's advantage over a batch library is structural**,
  not a constant factor. Its cost per tick is flat while a recompute-the-window
  approach grows with the window, so the gap widens as periods get longer.
- **For one indicator over one large array**, the difference is much smaller —
  both are doing a single pass. Use `extend` (one call over the array) rather
  than a Python loop of `append`, so the per-bar boundary cost is paid once.

If per-tick latency in Python matters more than anything, batch your ticks:
`extend` over a small array amortizes the boundary crossing across the bars in
it.

## Threading

Bulk work releases the GIL, so independent indicators on separate threads
compute in parallel:

```python
from concurrent.futures import ThreadPoolExecutor

def run(symbol_bars):
    ind = SimpleMovingAverage(timeperiod=30)
    ind.extend(symbol_bars)
    return ind.compute()

with ThreadPoolExecutor(max_workers=4) as pool:
    results = list(pool.map(run, per_symbol_arrays))
```

Scalar `append` deliberately does **not** release the GIL — the call is far too
short for that to pay off. Give each thread its own indicator instances;
a single indicator is not safe to update from two threads at once.

## Related

- [Indicator reference](INDICATORS.md) — every class, parameters, constructor
  order.
- [Data in and out](DATA.md) — accepted input containers and output converters.
- [Pipelines](PIPELINES.md) — stream many indicators through one graph, each
  bar dispatched once.
