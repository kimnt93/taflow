# RollingMedian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.24M | 0.036 | 27.91M | nan | — | — |
| 10,000 | 0.398 | 25.13M | 0.392 | 25.53M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.057 ms**; native kernel **0.062 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.261 | 0.194 | 5.16M | nan | — | — |
| 1,500 | 10 | 1.292 | 0.889 | 11.24M | nan | — | — |
| 1,500 | 100 | 5.070 | 4.821 | 20.74M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.84M | 9.54M | 1.00× | 1.22M | 1.09M | 1.00× | — |
| 2 | 14.14M | 12.28M | 1.29× | 1.16M | 1.35M | 1.23× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
