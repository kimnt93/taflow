# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 292.83M | 0.003 | 399.04M | 0.003 | 0.85× | 1.15× |
| 10,000 | 0.008 | 1.33G | 0.005 | 2.13G | 0.013 | 1.74× | 2.78× |
| 100,000 | 0.056 | 1.80G | 0.031 | 3.23G | 0.116 | 2.08× | 3.75× |
| 1,000,000 | 0.911 | 1.10G | 0.537 | 1.86G | 1.337 | 1.47× | 2.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.076 | 0.69× |
| 1 | 5 | 0.278 | 0.287 | 1.03× |
| 1 | 10 | 0.456 | 0.556 | 1.22× |
| 10 | 1 | 0.050 | 0.042 | 0.83× |
| 10 | 5 | 0.248 | 0.222 | 0.90× |
| 10 | 10 | 0.497 | 0.441 | 0.89× |
| 100 | 1 | 0.050 | 0.044 | 0.89× |
| 100 | 5 | 0.230 | 0.197 | 0.85× |
| 100 | 10 | 0.515 | 0.431 | 0.84× |
| 1,000 | 1 | 0.052 | 0.043 | 0.84× |
| 1,000 | 5 | 0.244 | 0.216 | 0.89× |
| 1,000 | 10 | 0.478 | 0.455 | 0.95× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.288 | 0.159 | 6.30M | nan | — | — |
| 100,000 | 10 | 1.041 | 0.533 | 18.77M | nan | — | — |
| 100,000 | 1,000 | 3.026 | 1.830 | 546.58M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 373.42M | 1.10G | 1.00× | 3.00M | 3.96M | 1.00× | — |
| 5 | 544.11M | 1.82G | 1.65× | 2.70M | 3.60M | 0.91× | — |
| 10 | 560.21M | 1.47G | 1.34× | 2.54M | 3.09M | 0.78× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
