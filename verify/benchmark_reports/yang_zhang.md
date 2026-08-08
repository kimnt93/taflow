# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.07M | 0.042 | 23.77M | nan | — | — |
| 10,000 | 0.406 | 24.66M | 0.396 | 25.28M | nan | — | — |
| 100,000 | 4.020 | 24.88M | 3.946 | 25.34M | nan | — | — |
| 1,000,000 | 40.463 | 24.71M | 39.700 | 25.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.102 ms**; native kernel **4.152 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.418 | 0.297 | 3.36M | nan | — | — |
| 100,000 | 10 | 2.790 | 1.508 | 6.63M | nan | — | — |
| 100,000 | 1,000 | 50.034 | 47.680 | 20.97M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.03M | 23.17M | 1.00× | 1.92M | 1.89M | 1.00× | — |
| 2 | 46.00M | 46.09M | 1.99× | 2.01M | 2.16M | 1.14× | — |
| 4 | 77.40M | 89.25M | 3.85× | 1.97M | 2.08M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
