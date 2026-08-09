# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.70M | 0.047 | 21.22M | nan | — | — |
| 10,000 | 0.461 | 21.70M | 0.506 | 19.75M | nan | — | — |
| 100,000 | 4.679 | 21.37M | 4.714 | 21.21M | nan | — | — |
| 1,000,000 | 47.598 | 21.01M | 48.456 | 20.64M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.708 ms**; native kernel **4.768 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.254 | 0.210 | 4.76M | nan | — | — |
| 100,000 | 10 | 1.375 | 1.067 | 9.37M | nan | — | — |
| 100,000 | 1,000 | 50.936 | 49.178 | 20.33M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18.39M | 19.27M | 1.00× | 2.93M | 2.42M | 1.00× | — |
| 2 | 38.88M | 41.08M | 2.13× | 2.69M | 2.81M | 1.16× | — |
| 4 | 58.67M | 64.50M | 3.35× | 2.68M | 2.75M | 1.14× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
