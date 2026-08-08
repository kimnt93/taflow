# EqualHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.44M | 0.044 | 22.58M | nan | — | — |
| 10,000 | 0.436 | 22.94M | 0.418 | 23.95M | nan | — | — |
| 100,000 | 4.353 | 22.97M | 4.542 | 22.02M | nan | — | — |
| 1,000,000 | 56.530 | 17.69M | 43.514 | 22.98M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.322 ms**; native kernel **4.197 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.371 | 0.326 | 3.07M | nan | — | — |
| 100,000 | 10 | 2.414 | 1.415 | 7.07M | nan | — | — |
| 100,000 | 1,000 | 51.436 | 47.876 | 20.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.17M | 22.49M | 1.00× | 1.94M | 1.83M | 1.00× | — |
| 2 | 40.31M | 39.57M | 1.76× | 1.92M | 1.98M | 1.08× | — |
| 4 | 67.34M | 73.71M | 3.28× | 2.08M | 2.01M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
