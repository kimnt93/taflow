# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.90M | 0.038 | 26.28M | nan | — | — |
| 10,000 | 0.432 | 23.15M | 0.418 | 23.93M | nan | — | — |
| 100,000 | 4.608 | 21.70M | 4.448 | 22.48M | nan | — | — |
| 1,000,000 | 61.107 | 16.36M | 47.863 | 20.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.499 ms**; native kernel **4.315 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.342 | 0.282 | 3.54M | nan | — | — |
| 100,000 | 10 | 1.947 | 1.211 | 8.25M | nan | — | — |
| 100,000 | 1,000 | 47.023 | 44.887 | 22.28M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 20.08M | 22.70M | 1.00× | 2.40M | 2.28M | 1.00× | — |
| 2 | 25.79M | 25.41M | 1.12× | 2.08M | 2.08M | 0.91× | — |
| 4 | 37.98M | 39.69M | 1.75× | 2.23M | 2.17M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
