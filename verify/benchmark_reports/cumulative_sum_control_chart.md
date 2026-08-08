# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.62M | 0.005 | 218.14M | nan | — | — |
| 10,000 | 0.039 | 259.59M | 0.036 | 281.46M | nan | — | — |
| 100,000 | 0.362 | 275.95M | 0.341 | 293.58M | nan | — | — |
| 1,000,000 | 3.957 | 252.72M | 3.477 | 287.57M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.370 ms**; native kernel **0.330 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.212 | 0.152 | 6.60M | nan | — | — |
| 100,000 | 10 | 0.876 | 0.535 | 18.69M | nan | — | — |
| 100,000 | 1,000 | 5.392 | 4.792 | 208.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 196.28M | 205.08M | 1.00× | 2.95M | 2.94M | 1.00× | — |
| 2 | 337.18M | 443.63M | 2.16× | 3.65M | 3.73M | 1.27× | — |
| 4 | 516.07M | 798.27M | 3.89× | 3.77M | 4.04M | 1.37× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
