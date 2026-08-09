# RollingQuantile benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.89M | 0.044 | 22.78M | nan | — | — |
| 10,000 | 0.481 | 20.80M | 0.483 | 20.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.068 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.280 | 0.216 | 4.62M | nan | — | — |
| 1,500 | 10 | 1.430 | 0.987 | 10.13M | nan | — | — |
| 1,500 | 100 | 6.104 | 5.485 | 18.23M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.60M | 8.71M | 1.00× | 1.34M | 881.84K | 1.00× | — |
| 2 | 12.41M | 11.96M | 1.37× | 1.39M | 1.47M | 1.66× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
