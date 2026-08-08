# ExponentiallyWeightedCorrelation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.71M | 0.007 | 153.12M | nan | — | — |
| 10,000 | 0.057 | 174.04M | 0.053 | 189.40M | nan | — | — |
| 100,000 | 0.551 | 181.54M | 0.533 | 187.62M | nan | — | — |
| 1,000,000 | 6.020 | 166.10M | 5.502 | 181.75M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.560 ms**; native kernel **0.516 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.261 | 0.195 | 5.13M | nan | — | — |
| 100,000 | 10 | 1.526 | 0.813 | 12.29M | nan | — | — |
| 100,000 | 1,000 | 8.672 | 6.972 | 143.43M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.61M | 147.75M | 1.00× | 3.27M | 2.87M | 1.00× | — |
| 2 | 244.60M | 249.31M | 1.69× | 2.69M | 3.18M | 1.11× | — |
| 4 | 361.64M | 510.35M | 3.45× | 3.03M | 3.09M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
