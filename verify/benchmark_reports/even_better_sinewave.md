# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.42M | 0.006 | 168.00M | nan | — | — |
| 10,000 | 0.052 | 193.27M | 0.049 | 202.23M | nan | — | — |
| 100,000 | 0.492 | 203.34M | 0.469 | 213.23M | nan | — | — |
| 1,000,000 | 5.283 | 189.30M | 4.951 | 201.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.490 ms**; native kernel **0.479 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.214 | 0.147 | 6.82M | nan | — | — |
| 100,000 | 10 | 0.619 | 0.434 | 23.04M | nan | — | — |
| 100,000 | 1,000 | 6.368 | 5.774 | 173.18M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 141.62M | 164.74M | 1.00× | 2.94M | 2.97M | 1.00× | — |
| 2 | 161.70M | 174.87M | 1.06× | 3.29M | 3.52M | 1.18× | — |
| 4 | 160.18M | 173.43M | 1.05× | 3.25M | 3.48M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
