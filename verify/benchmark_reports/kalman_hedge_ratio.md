# KalmanHedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.34M | 0.016 | 64.22M | nan | — | — |
| 10,000 | 0.152 | 65.95M | 0.148 | 67.58M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.025 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.304 | 0.215 | 4.65M | nan | — | — |
| 1,500 | 10 | 1.599 | 0.794 | 12.59M | nan | — | — |
| 1,500 | 100 | 3.911 | 3.012 | 33.20M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
