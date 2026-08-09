# PositiveVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.31M | 0.012 | 81.14M | nan | — | — |
| 10,000 | 0.070 | 142.89M | 0.065 | 154.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.409 | 0.229 | 4.37M | nan | — | — |
| 1,500 | 10 | 1.752 | 1.528 | 6.55M | nan | — | — |
| 1,500 | 100 | 3.192 | 2.248 | 44.48M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 14.89M | 16.89M | 1.00× | 1.24M | 1.39M | 1.00× | — |
| 2 | 20.19M | 21.01M | 1.24× | 1.55M | 1.62M | 1.16× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
