# GapDown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.77M | 0.006 | 161.81M | nan | — | — |
| 10,000 | 0.036 | 274.57M | 0.032 | 308.62M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.347 | 0.195 | 5.14M | nan | — | — |
| 1,500 | 10 | 1.518 | 0.738 | 13.55M | nan | — | — |
| 1,500 | 100 | 2.506 | 1.547 | 64.63M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.57M | 8.55M | 1.00× | 1.20M | 1.03M | 1.00× | — |
| 2 | 15.47M | 16.76M | 1.96× | 1.24M | 1.38M | 1.35× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
