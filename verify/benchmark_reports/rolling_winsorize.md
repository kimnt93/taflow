# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.29M | 0.058 | 17.24M | nan | — | — |
| 10,000 | 0.590 | 16.94M | 0.600 | 16.67M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.088 ms**; native kernel **0.093 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.280 | 0.222 | 4.49M | nan | — | — |
| 1,500 | 10 | 1.966 | 1.043 | 9.59M | nan | — | — |
| 1,500 | 100 | 6.657 | 6.351 | 15.75M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.17M | 5.18M | 1.00× | 962.40K | 814.34K | 1.00× | — |
| 2 | 12.46M | 12.62M | 2.44× | 1.42M | 1.53M | 1.87× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
