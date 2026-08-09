# RollingQuantile benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.32M | 0.042 | 23.63M | nan | — | — |
| 10,000 | 0.391 | 25.60M | 0.382 | 26.18M | nan | — | — |
| 100,000 | 3.820 | 26.18M | 3.766 | 26.55M | nan | — | — |
| 1,000,000 | 39.557 | 25.28M | 39.631 | 25.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.777 ms**; native kernel **3.795 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.237 | 0.191 | 5.24M | nan | — | — |
| 100,000 | 10 | 1.217 | 0.851 | 11.75M | nan | — | — |
| 100,000 | 1,000 | 51.739 | 41.688 | 23.99M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.26M | 24.88M | 1.00× | 2.66M | 2.47M | 1.00× | — |
| 2 | 47.95M | 48.24M | 1.94× | 3.04M | 2.86M | 1.15× | — |
| 4 | 85.53M | 94.17M | 3.78× | 2.79M | 2.89M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
