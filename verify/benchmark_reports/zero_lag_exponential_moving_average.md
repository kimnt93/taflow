# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.58M | 0.005 | 189.50M | nan | — | — |
| 10,000 | 0.047 | 214.68M | 0.043 | 233.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.290 | 0.189 | 5.28M | nan | — | — |
| 1,500 | 10 | 1.118 | 0.607 | 16.47M | nan | — | — |
| 1,500 | 100 | 2.885 | 2.083 | 48.01M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
