# HullMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.03M | 0.024 | 41.24M | nan | — | — |
| 10,000 | 0.218 | 45.93M | 0.213 | 46.87M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.036 ms**; native kernel **0.036 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.254 | 0.185 | 5.39M | nan | — | — |
| 1,500 | 10 | 1.785 | 0.731 | 13.69M | nan | — | — |
| 1,500 | 100 | 3.633 | 3.154 | 31.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.55M | 8.46M | 1.00× | 1.21M | 1.48M | 1.00× | — |
| 2 | 17.23M | 17.62M | 2.08× | 1.54M | 1.69M | 1.14× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
