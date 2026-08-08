# HullMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.99M | 0.017 | 60.41M | nan | — | — |
| 10,000 | 0.158 | 63.28M | 0.159 | 63.01M | nan | — | — |
| 100,000 | 1.561 | 64.05M | 1.490 | 67.13M | nan | — | — |
| 1,000,000 | 15.831 | 63.17M | 15.059 | 66.41M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.530 ms**; native kernel **1.509 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.212 | 4.72M | nan | — | — |
| 100,000 | 10 | 1.304 | 0.805 | 12.42M | nan | — | — |
| 100,000 | 1,000 | 20.925 | 21.285 | 46.98M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 57.95M | 61.49M | 1.00× | 2.65M | 2.98M | 1.00× | — |
| 2 | 119.23M | 128.17M | 2.08× | 3.65M | 3.49M | 1.17× | — |
| 4 | 204.35M | 240.71M | 3.91× | 3.02M | 3.02M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
