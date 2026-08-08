# Falling benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.20M | 0.006 | 171.94M | nan | — | — |
| 10,000 | 0.051 | 197.98M | 0.048 | 210.34M | nan | — | — |
| 100,000 | 0.473 | 211.21M | 0.499 | 200.29M | nan | — | — |
| 1,000,000 | 4.998 | 200.09M | 4.866 | 205.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.470 ms**; native kernel **0.443 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.155 | 6.44M | nan | — | — |
| 100,000 | 10 | 0.943 | 0.569 | 17.56M | nan | — | — |
| 100,000 | 1,000 | 6.937 | 5.707 | 175.23M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.60M | 200.23M | 1.00× | 3.03M | 3.20M | 1.00× | — |
| 2 | 307.77M | 362.67M | 1.81× | 3.29M | 3.72M | 1.16× | — |
| 4 | 216.02M | 228.96M | 1.14× | 3.33M | 3.66M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
