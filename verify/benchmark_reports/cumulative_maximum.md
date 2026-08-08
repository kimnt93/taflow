# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.89M | 0.005 | 203.59M | nan | — | — |
| 10,000 | 0.044 | 225.18M | 0.041 | 245.95M | nan | — | — |
| 100,000 | 0.443 | 225.48M | 0.392 | 254.91M | nan | — | — |
| 1,000,000 | 4.718 | 211.98M | 4.205 | 237.79M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.429 ms**; native kernel **0.398 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.206 | 0.157 | 6.38M | nan | — | — |
| 100,000 | 10 | 0.878 | 0.559 | 17.89M | nan | — | — |
| 100,000 | 1,000 | 6.030 | 5.067 | 197.35M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 196.92M | 206.42M | 1.00× | 3.52M | 3.67M | 1.00× | — |
| 2 | 287.54M | 381.14M | 1.85× | 3.40M | 3.27M | 0.89× | — |
| 4 | 444.87M | 732.24M | 3.55× | 3.82M | 3.78M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
