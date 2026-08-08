# Sessions benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.61M | 0.009 | 105.77M | nan | — | — |
| 10,000 | 0.084 | 119.23M | 0.073 | 137.33M | nan | — | — |
| 100,000 | 0.768 | 130.18M | 0.701 | 142.63M | nan | — | — |
| 1,000,000 | 21.008 | 47.60M | 8.477 | 117.96M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.770 ms**; native kernel **0.727 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.429 | 0.372 | 2.69M | nan | — | — |
| 100,000 | 10 | 1.768 | 0.985 | 10.15M | nan | — | — |
| 100,000 | 1,000 | 9.744 | 9.273 | 107.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 100.75M | 117.49M | 1.00× | 1.77M | 1.86M | 1.00× | — |
| 2 | 151.09M | 163.29M | 1.39× | 1.96M | 1.91M | 1.02× | — |
| 4 | 160.41M | 199.92M | 1.70× | 1.91M | 1.96M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
