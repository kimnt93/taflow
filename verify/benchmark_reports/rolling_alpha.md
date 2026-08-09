# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.95M | 0.044 | 22.93M | nan | — | — |
| 10,000 | 0.420 | 23.79M | 0.417 | 23.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.064 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.322 | 0.245 | 4.07M | nan | — | — |
| 1,500 | 10 | 1.958 | 1.200 | 8.33M | nan | — | — |
| 1,500 | 100 | 6.336 | 5.613 | 17.81M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.81M | 10.04M | 1.00× | 1.31M | 910.38K | 1.00× | — |
| 2 | 13.70M | 13.49M | 1.34× | 1.42M | 1.34M | 1.48× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
