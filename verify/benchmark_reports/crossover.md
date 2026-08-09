# Crossover benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 202.84M | 0.004 | 277.22M | nan | — | — |
| 10,000 | 0.031 | 318.74M | 0.028 | 354.52M | nan | — | — |
| 100,000 | 0.294 | 339.83M | 0.267 | 374.34M | nan | — | — |
| 1,000,000 | 3.525 | 283.67M | 2.998 | 333.58M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.287 ms**; native kernel **0.261 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.237 | 0.183 | 5.48M | nan | — | — |
| 100,000 | 10 | 1.378 | 0.663 | 15.07M | nan | — | — |
| 100,000 | 1,000 | 5.094 | 4.083 | 244.90M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 178.89M | 223.78M | 1.00× | 3.20M | 3.08M | 1.00× | — |
| 2 | 374.23M | 212.98M | 0.95× | 3.24M | 3.23M | 1.05× | — |
| 4 | 323.31M | 371.35M | 1.66× | 3.01M | 3.45M | 1.12× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
