# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.27M | 0.043 | 23.11M | nan | — | — |
| 10,000 | 0.426 | 23.48M | 0.414 | 24.18M | nan | — | — |
| 100,000 | 4.175 | 23.95M | 4.138 | 24.17M | nan | — | — |
| 1,000,000 | 42.011 | 23.80M | 43.312 | 23.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.214 ms**; native kernel **4.149 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.285 | 3.51M | nan | — | — |
| 100,000 | 10 | 2.876 | 1.450 | 6.89M | nan | — | — |
| 100,000 | 1,000 | 46.116 | 43.626 | 22.92M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 19.30M | 22.84M | 1.00× | 2.02M | 2.06M | 1.00× | — |
| 2 | 42.10M | 44.10M | 1.93× | 2.22M | 2.24M | 1.09× | — |
| 4 | 75.77M | 83.05M | 3.64× | 2.07M | 2.24M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
