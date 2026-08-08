# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.86M | 0.052 | 19.18M | nan | — | — |
| 10,000 | 0.588 | 17.00M | 0.600 | 16.66M | nan | — | — |
| 100,000 | 6.027 | 16.59M | 6.016 | 16.62M | nan | — | — |
| 1,000,000 | 61.343 | 16.30M | 59.908 | 16.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.116 ms**; native kernel **6.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.288 | 0.226 | 4.42M | nan | — | — |
| 100,000 | 10 | 1.329 | 1.127 | 8.87M | nan | — | — |
| 100,000 | 1,000 | 59.017 | 57.411 | 17.42M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.80M | 16.02M | 1.00× | 2.19M | 2.63M | 1.00× | — |
| 2 | 29.94M | 31.39M | 1.96× | 2.26M | 2.31M | 0.88× | — |
| 4 | 55.70M | 58.75M | 3.67× | 2.14M | 2.18M | 0.83× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
