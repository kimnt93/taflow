# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.62M | 0.049 | 20.48M | nan | — | — |
| 10,000 | 0.429 | 23.32M | 0.389 | 25.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.064 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.431 | 0.369 | 2.71M | nan | — | — |
| 1,500 | 10 | 2.361 | 1.328 | 7.53M | nan | — | — |
| 1,500 | 100 | 6.775 | 5.581 | 17.92M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.76M | 8.30M | 1.00× | 1.13M | 1.04M | 1.00× | — |
| 2 | 12.00M | 13.74M | 1.66× | 1.22M | 1.29M | 1.24× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
