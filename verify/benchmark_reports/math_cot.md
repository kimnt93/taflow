# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.03M | 0.018 | 56.54M | nan | — | — |
| 10,000 | 0.204 | 49.03M | 0.201 | 49.63M | nan | — | — |
| 100,000 | 2.039 | 49.05M | 2.010 | 49.76M | nan | — | — |
| 1,000,000 | 21.392 | 46.75M | 20.994 | 47.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.063 ms**; native kernel **2.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.239 | 0.185 | 5.41M | nan | — | — |
| 100,000 | 10 | 1.150 | 0.749 | 13.35M | nan | — | — |
| 100,000 | 1,000 | 22.944 | 21.646 | 46.20M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.43M | 44.30M | 1.00× | 2.69M | 2.40M | 1.00× | — |
| 2 | 78.42M | 83.97M | 1.90× | 2.80M | 3.14M | 1.31× | — |
| 4 | 125.29M | 160.13M | 3.61× | 2.45M | 2.94M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
