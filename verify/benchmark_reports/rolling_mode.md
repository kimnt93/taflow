# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.107 | 9.38M | 0.108 | 9.27M | nan | — | — |
| 10,000 | 1.106 | 9.04M | 1.078 | 9.27M | nan | — | — |
| 100,000 | 10.779 | 9.28M | 10.801 | 9.26M | nan | — | — |
| 1,000,000 | 104.427 | 9.58M | 102.831 | 9.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **10.688 ms**; native kernel **10.353 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.346 | 0.365 | 2.74M | nan | — | — |
| 100,000 | 10 | 1.958 | 1.628 | 6.14M | nan | — | — |
| 100,000 | 1,000 | 115.747 | 99.720 | 10.03M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.10M | 9.95M | 1.00× | 2.35M | 2.28M | 1.00× | — |
| 2 | 18.69M | 19.01M | 1.91× | 2.34M | 2.40M | 1.05× | — |
| 4 | 35.23M | 36.37M | 3.66× | 2.44M | 2.29M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
