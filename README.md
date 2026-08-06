<p align="center">
  <h1 align="center">TAFlow</h1>
  <p align="center">
    Pure Rust Technical Analysis Library — Drop-in Replacement for TA-Lib
  </p>
  <p align="center">
    <a href="README.zh-CN.md">中文</a> · <a href="BENCHMARK.md">Benchmarks</a>
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/indicators-161-blue" alt="161 indicators" />
  <img src="https://img.shields.io/badge/tests-4562_accuracy-green" alt="17855 accuracy tests" />
  <img src="https://img.shields.io/badge/unsafe-zero-brightgreen" alt="zero unsafe" />
  <img src="https://img.shields.io/badge/precision-bit--exact-brightgreen" alt="bit-exact" />
  <img src="https://img.shields.io/badge/C_deps-zero-orange" alt="zero C deps" />
  <img src="https://img.shields.io/badge/license-BSD--3--Clause-lightgrey" alt="BSD-3-Clause" />
</p>

---

## Why TAFlow?

[TA-Lib](https://ta-lib.org/) has been the industry standard for technical analysis since 1999, but its C implementation brings well-known pain points: difficult compilation on Windows/macOS, fragile Python wrapper builds, and no modern language safety guarantees. **TAFlow** solves all of these while delivering identical results.

| | C TA-Lib | TAFlow |
|---|---|---|
| Language | C (1999) | Rust (2024) |
| Installation | Requires C compiler + system libs | `pip install taflow` |
| Accuracy | Reference implementation | **Bit-exact match** (17,855 accuracy tests × 7 data sizes × 6 scenarios) |
| Performance | Baseline | **1.41x avg faster** at 1M bars (47/90 faster, zero unsafe) |
| Memory safety | Manual management | Guaranteed by Rust |
| Python integration | Cython wrapper | PyO3 zero-copy |
| Indicators | 161 | 161 names exposed (batch compatibility audit in progress) |

### Key Technical Advantages

- **Bit-exact precision** — Not "approximately equal", but `diff=0` against C TA-Lib across 1M+ data points. Verified with 353 accuracy tests covering every function, every parameter combination, and extreme edge cases.

- **O(n) algorithms everywhere** — STDDEV, CORREL, BETA, LINEARREG, KAMA, TRIMA all use incremental sliding-window formulas. Period-independent performance: `CORREL(period=200)` runs in the same time as `CORREL(period=10)`.

- **SIMD acceleration** — `wide` crate f64x4 vectorization for reduction operations (sum, sum-of-squares). ~3x speedup on aggregate computations used by SMA, BBANDS, STDDEV, VAR.

- **Zero-copy NumPy bridge** — Input arrays read directly from NumPy memory via `PyReadonlyArray1::as_slice()`. Output arrays transferred via `PyArray1::from_vec()` with ownership move, no memcpy.

- **Monotonic deque optimization** — AROON, WILLR, MAX, MIN, MIDPOINT, MIDPRICE use O(n) sliding-window extrema instead of O(n×p) brute-force scan.

- **Zero unsafe** — Entire codebase uses safe Rust. No `unsafe` blocks, no `get_unchecked`. Iterator-based patterns enable LLVM auto-vectorization while maintaining full bounds safety.

## Installation

```bash
pip install taflow
```

No C compiler, no system libraries, no build failures. Pre-built wheels for Linux/macOS/Windows × Python 3.9–3.13.

## Quick Start

### Function API (TA-Lib compatible)

```python
from taflow import talib
import numpy as np

close = np.random.random(1000) * 100

# Trend
sma  = talib.SMA(close, timeperiod=20)
ema  = talib.EMA(close, timeperiod=12)
kama = talib.KAMA(close, timeperiod=30)
upper, mid, lower = talib.BBANDS(close, timeperiod=20, nbdevup=2, nbdevdn=2)

# Momentum
rsi  = talib.RSI(close, timeperiod=14)
macd, signal, hist = talib.MACD(close, fastperiod=12, slowperiod=26, signalperiod=9)
slowk, slowd = talib.STOCH(high, low, close, fastk_period=5, slowk_period=3)

# Volatility & Volume
atr = talib.ATR(high, low, close, timeperiod=14)
obv = talib.OBV(close, volume)

# Pattern Recognition (61 candlestick patterns)
engulfing = talib.CDLENGULFING(open, high, low, close)  # -100 / 0 / 100
hammer    = talib.CDLHAMMER(open, high, low, close)

# Metadata
print(len(talib.get_functions()))       # 161
print(talib.get_function_groups().keys())
# ['Overlap Studies', 'Momentum Indicators', 'Pattern Recognition', ...]
```

### Descriptive stateful API

Each descriptive indicator is defined in its own Python module and keeps its
Rust state between updates:

```python
from taflow import (
    BollingerBands,
    FastStochasticOscillator,
    IntradayMomentumIndex,
    MovingAverage,
    MovingAverageConvergenceDivergenceExtended,
    MovingAverageConvergenceDivergenceFixed,
    StochasticOscillator,
    StochasticRelativeStrengthIndex,
    VariablePeriodMovingAverage,
)

average = MovingAverage(period=20, moving_average_type=1)
average.extend(history)
latest_average = average.append(next_close)

bands = BollingerBands(period=20, deviations_up=2.0, deviations_down=2.0)
upper, middle, lower = bands.extend(history)

imi = IntradayMomentumIndex(period=14)
imi.extend(open_history, close_history)
latest_imi = imi.append(next_open, next_close)

macd_fixed = MovingAverageConvergenceDivergenceFixed(signal_period=9)
macd_fixed.extend(history)
macd, signal, histogram = macd_fixed.append(next_close)

macd_extended = MovingAverageConvergenceDivergenceExtended(
    fast_period=7,
    fast_average_type=1,
    slow_period=13,
    slow_average_type=1,
    signal_period=5,
    signal_average_type=1,
)
macd, signal, histogram = macd_extended.extend(history)

variable_average = VariablePeriodMovingAverage(
    min_period=2,
    max_period=30,
    average_type=0,
)
values = variable_average.extend(history, period_history)

stochastic = FastStochasticOscillator(
    fast_k_period=5,
    fast_d_period=13,
    fast_d_average_type=0,
)
fast_k, fast_d = stochastic.extend(high_history, low_history, close_history)

slow_stochastic = StochasticOscillator(
    fast_k_period=5,
    slow_k_period=13,
    slow_d_period=11,
)
slow_k, slow_d = slow_stochastic.extend(
    high_history, low_history, close_history
)

stochastic_rsi = StochasticRelativeStrengthIndex(
    time_period=14,
    fast_k_period=5,
    fast_d_period=13,
)
fast_k, fast_d = stochastic_rsi.extend(history)
```

Use `taflow.talib` for uppercase TA-Lib-compatible batch calls and top-level
`taflow` for descriptive persistent streaming classes.

### Abstract API

```python
from taflow.talib.abstract import Function

# Create function with introspection
rsi = Function('RSI')
print(rsi.info['group'])         # 'Momentum Indicators'
print(rsi.input_names)           # OrderedDict([('price', ['close'])])
print(rsi.parameters)            # OrderedDict([('timeperiod', 14)])
print(rsi.output_names)          # ['real']
print(rsi.lookback)              # 14

# Call with dict or DataFrame
data = {'close': close, 'high': high, 'low': low, 'open': open_, 'volume': volume}
result = rsi(data)

# Override parameters
rsi.parameters = {'timeperiod': 21}
result = rsi(data)

# One-liner imports
from taflow.talib.abstract import SMA, MACD, BBANDS
sma_result = SMA(data, timeperiod=50)
```

### Latest-value compatibility API

```python
from taflow.talib import stream

# Returns only the latest value (scalar), not the full array
latest_rsi  = stream.RSI(close, timeperiod=14)          # float
latest_macd = stream.MACD(close, 12, 26, 9)             # (float, float, float)
latest_sma  = stream.SMA(close, timeperiod=20)           # float
```

This compatibility module has TA-Lib's latest-value calling convention: it
accepts complete history and returns its final value.  For persistent,
incremental O(1)-per-bar state, use the stateful API:

```python
from taflow.talib.state import RSI, MACD

rsi = RSI(14)
rsi.extend(history)           # NumPy array, with NaN during warm-up
latest_rsi = rsi.append(tick) # float once warm, otherwise None

macd = MACD(12, 26, 9)
macd.extend(history)
line, signal, histogram = macd.append(tick)
```

## Performance

> Apple M4 | `--release` LTO fat | median of 20 iterations | **zero unsafe** code

| Dataset | Faster | Equal (±5%) | Slower | Avg | Median |
|--------:|:------:|:-----------:|:------:|:---:|:------:|
| 1,000 | 31 | 26 | 33 | 1.16x | 1.00x |
| 10,000 | 38 | 34 | 18 | 1.34x | 1.02x |
| 100,000 | **45** | 39 | 6 | **1.40x** | **1.06x** |
| 1,000,000 | **47** | 35 | 8 | **1.41x** | **1.07x** |

### Highlights at 1M bars

| Indicator | C (us) | Rust (us) | Speedup | Technique |
|-----------|-------:|---------:|--------:|-----------|
| MIDPOINT(14) | 39,300 | 3,720 | **10.6x** | O(n) cached index vs C's O(n×p) brute scan |
| BBANDS(20) | 3,500 | 1,160 | **3.0x** | Single-pass fused SMA+STDDEV |
| TEMA(20) | 3,900 | 1,420 | **2.8x** | Inline 3-layer EMA cascade, no intermediate Vec |
| LINEARREG(14) | 4,020 | 1,830 | **2.2x** | O(n) sliding sums vs C's O(n×p) |
| TRIX(15) | 4,100 | 1,940 | **2.1x** | Fused 3-layer EMA + ROC, C-style formulation |
| OBV | 3,160 | 1,600 | **2.0x** | Sequential push avoids calloc COW page faults |
| DEMA(20) | 2,640 | 1,390 | **1.9x** | Eliminated intermediate ema1 Vec |
| STOCH | 6,680 | 3,720 | **1.8x** | Sliding window min/max + EMA signal |
| VAR(20) | 1,160 | 710 | **1.6x** | O(n) sliding sum + div→mul (precompute 1/n) |
| STDDEV(20) | 1,400 | 870 | **1.6x** | Fused var+sqrt single-pass |
| MACD | 4,260 | 2,790 | **1.5x** | C TA-Lib EMA formulation: `k*(x-prev)+prev` |

Full benchmark: 90 indicators × 4 datasets with C/Rust times → [BENCHMARK.md](BENCHMARK.md)

## Indicators (161)

### Overlap Studies (17)
ACCBANDS, SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA, T3, MAMA, BBANDS, SAR, SAREXT, MIDPOINT, MIDPRICE, MAVP, HT_TRENDLINE

### Momentum Indicators (31)
IMI, RSI, MACD, MACDEXT, MACDFIX, STOCH, STOCHF, STOCHRSI, ADX, ADXR, CCI, MOM, ROC, ROCP, ROCR, ROCR100, WILLR, APO, PPO, BOP, CMO, AROON, AROONOSC, MFI, TRIX, ULTOSC, DX, PLUS_DI, MINUS_DI, PLUS_DM, MINUS_DM

### Pattern Recognition (61)
CDL2CROWS, CDL3BLACKCROWS, CDL3INSIDE, CDL3LINESTRIKE, CDL3OUTSIDE, CDL3STARSINSOUTH, CDL3WHITESOLDIERS, CDLABANDONEDBABY, CDLADVANCEBLOCK, CDLBELTHOLD, CDLBREAKAWAY, CDLCLOSINGMARUBOZU, CDLCONCEALBABYSWALL, CDLCOUNTERATTACK, CDLDARKCLOUDCOVER, CDLDOJI, CDLDOJISTAR, CDLDRAGONFLYDOJI, CDLENGULFING, CDLEVENINGDOJISTAR, CDLEVENINGSTAR, CDLGAPSIDESIDEWHITE, CDLGRAVESTONEDOJI, CDLHAMMER, CDLHANGINGMAN, CDLHARAMI, CDLHARAMICROSS, CDLHIGHWAVE, CDLHIKKAKE, CDLHIKKAKEMOD, CDLHOMINGPIGEON, CDLIDENTICAL3CROWS, CDLINNECK, CDLINVERTEDHAMMER, CDLKICKING, CDLKICKINGBYLENGTH, CDLLADDERBOTTOM, CDLLONGLEGGEDDOJI, CDLLONGLINE, CDLMARUBOZU, CDLMATCHINGLOW, CDLMATHOLD, CDLMORNINGDOJISTAR, CDLMORNINGSTAR, CDLONNECK, CDLPIERCING, CDLRICKSHAWMAN, CDLRISEFALL3METHODS, CDLSEPARATINGLINES, CDLSHOOTINGSTAR, CDLSHORTLINE, CDLSPINNINGTOP, CDLSTALLEDPATTERN, CDLSTICKSANDWICH, CDLTAKURI, CDLTASUKIGAP, CDLTHRUSTING, CDLTRISTAR, CDLUNIQUE3RIVER, CDLUPSIDEGAP2CROWS, CDLXSIDEGAP3METHODS

### Volatility (3)
ATR, NATR, TRANGE

### Volume (3)
AD, ADOSC, OBV

### Price Transform (4)
AVGPRICE, MEDPRICE, TYPPRICE, WCLPRICE

### Statistic Functions (10)
AVGDEV, STDDEV, VAR, BETA, CORREL, LINEARREG, LINEARREG_SLOPE, LINEARREG_INTERCEPT, LINEARREG_ANGLE, TSF

### Math Transform (15)
ACOS, ASIN, ATAN, CEIL, COS, COSH, EXP, FLOOR, LN, LOG10, SIN, SINH, SQRT, TAN, TANH

### Math Operators (9)
ADD, SUB, MULT, DIV, MAX, MAXINDEX, MIN, MININDEX, SUM

### Cycle Indicators (5)
HT_DCPERIOD, HT_DCPHASE, HT_PHASOR, HT_SINE, HT_TRENDMODE

## Testing & Verification

```
158/158 functions · 17,855 accuracy tests · 7 data sizes × 6 scenarios × 3 seeds · 0 failures
```

### Test Matrix

| Suite | Cases | Description |
|-------|------:|------------|
| **Rust unit tests** | 54 | Core algorithm, SIMD, sliding window, edge cases |
| **Accuracy cross-validation** | 353 | 158 functions × 6 datasets, rtol=1e-10 |
| **Multi-dataset alignment** | 17,451 | 158 functions × 7 sizes × 6 scenarios × 3 seeds |
| **Pattern exact match** | 122 | 61 CDL patterns × 2 datasets, integer signal exact |

### Verification Methodology

1. **Tiered tolerance** — Exact (1e-14) for element-wise ops, standard (1e-10) for EMA chains, sliding (1e-8) for O(n) accumulators, accumulative (1e-6) for VAR/STDDEV.
2. **NaN alignment** — Lookback NaN positions match exactly between TAFlow and C TA-Lib.
3. **Multi-scenario** — Random walk, trending up/down, sideways, volatile, mean-reverting data.
4. **Pattern recognition** — All 61 candlestick patterns produce identical signals (-100/0/100).

```bash
cargo test                                               # 54 Rust unit tests
pytest tests/accuracy/ -v                                # 17,855 accuracy tests
pytest tests/accuracy/ -k "100000"                       # Only 100K dataset
pytest tests/accuracy/ -k "volatile"                     # Only volatile scenario
pytest tests/accuracy/test_multi_dataset_alignment.py    # Multi-dataset suite
```

## Architecture

```
TAFlow/
├── crates/
│   ├── taflow-core/                    # Pure Rust library (no Python dependency)
│   │   ├── src/
│   │   │   ├── overlap/               # SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA,
│   │   │   │                          # T3, MAMA, BBANDS, SAR, MIDPOINT, ...
│   │   │   ├── momentum/              # RSI, MACD, STOCH, ADX, CCI, MOM, ROC,
│   │   │   │                          # WILLR, AROON, MFI, TRIX, ULTOSC, DX, ...
│   │   │   ├── volatility/            # ATR, NATR, TRANGE
│   │   │   ├── volume/                # AD, ADOSC, OBV
│   │   │   ├── pattern/               # 61 candlestick patterns with real logic
│   │   │   ├── statistic/             # STDDEV, VAR, BETA, CORREL, LINEARREG, TSF
│   │   │   ├── cycle/                 # Hilbert Transform (ring buffer optimized)
│   │   │   ├── math_transform/        # sin, cos, sqrt, exp, ln, ... (loop-unrolled)
│   │   │   ├── math_operator/         # add, sub, max, min, sum (monotonic deque)
│   │   │   ├── price_transform/       # avgprice, medprice, typprice, wclprice
│   │   │   ├── simd.rs                # f64x4 SIMD: sum, sum_sq_diff (3x speedup)
│   │   │   ├── sliding_window.rs      # Monotonic deque: O(n) sliding max/min
│   │   │   ├── ma_type.rs             # MA type dispatcher (9 types)
│   │   │   └── error.rs               # TaError enum
│   │   └── benches/
│   │       └── simd_bench.rs          # Criterion benchmarks
│   │
│   └── taflow-python/                  # PyO3 bindings
│       └── src/
│           ├── func_api.rs            # 161 #[pyfunction] definitions
│           ├── metadata.rs            # get_functions(), get_function_groups()
│           └── conversion.rs          # Zero-copy NumPy ↔ Rust
│
├── python/taflow/
│   ├── __init__.py                    # descriptive stateful API
│   └── talib/                         # uppercase compatibility API
│   ├── abstract.py                    # Function class with introspection
│   └── stream.py                      # Latest-value-only wrappers
│
├── tests/
│   ├── test_exhaustive.py             # 161-function cross-validation
│   ├── test_full_coverage.py          # Consistency + edge (620 tests)
│   └── accuracy/                      # Focused accuracy tests
│
├── benches/
│   ├── generate_report.py             # → BENCHMARK.md
│   └── python_benches/                # pytest-benchmark suites
│
└── .github/workflows/
    ├── ci.yml                         # Test + lint (3 OS × 4 Python versions)
    └── release.yml                    # Build wheels + publish to PyPI
```

### Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| SIMD crate | `wide` 0.7 | Stable Rust, no nightly required |
| Sliding extrema | Monotonic deque | Fused dual-extremum scan, amortized O(n) |
| Variance | E[X²]−E[X]² | Single-pass sliding window, no Welford two-pass |
| Hilbert Transform | Ring buffer | 10 Vec → fixed [f64; 64], 80% less heap allocation |
| EMA/DEMA/TEMA/T3 | In-place layered | No intermediate Vec from NaN filtering |
| WMA | Incremental recurrence | WS_new = WS_old − S_old + p×x_new, O(1) per step |
| Python compatibility API | `taflow.talib` | `from taflow import talib` |
| Output format | `Vec<f64>` → `PyArray1` | Ownership transfer, near-zero-copy |
| Unsafe policy | Zero `unsafe` | Iterator patterns + safe indexing; LLVM auto-vectorizes zip chains |
| NaN convention | Fill lookback with NaN | Matches C TA-Lib exactly |

## Development

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Python dependencies
pip install maturin numpy pytest pytest-benchmark
```

### Build & Test

```bash
maturin develop --release              # Build Python package
cargo test --workspace                 # 68 Rust tests
pytest tests/ -q                       # 353 accuracy tests
```

### Benchmarking

```bash
cargo bench --bench simd_bench         # Rust SIMD micro-benchmarks
cargo bench --bench stream_bench       # Stateful append throughput
pytest benches/ --benchmark-sort=name  # Python vs C TA-Lib
python benches/generate_report.py      # Generate BENCHMARK.md
```

### CI/CD

- **CI**: Rust test + clippy + fmt + Python build on Ubuntu/macOS/Windows × Python 3.10–3.13
- **Release**: Tag `v*` triggers manylinux/macOS/Windows wheel builds + PyPI publish

## Migration from C TA-Lib

```bash
# Remove C TA-Lib
pip uninstall TA-Lib

# Install TAFlow
pip install taflow

# Update imports: from taflow import talib
```

## License

BSD-3-Clause
