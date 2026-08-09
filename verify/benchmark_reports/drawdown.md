# Drawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.97M | 0.006 | 157.95M | nan | — | — |
| 10,000 | 0.046 | 218.29M | 0.043 | 230.96M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.231 | 0.180 | 5.56M | nan | — | — |
| 1,500 | 10 | 0.959 | 0.554 | 18.05M | nan | — | — |
| 1,500 | 100 | 1.979 | 1.524 | 65.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.37M | 17.92M | 1.00× | 1.46M | 1.64M | 1.00× | — |
| 2 | 17.76M | 21.79M | 1.22× | 1.67M | 1.58M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
