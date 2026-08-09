# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.091 | 11.01M | 0.091 | 11.05M | nan | — | — |
| 10,000 | 0.885 | 11.30M | 0.859 | 11.64M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.133 ms**; native kernel **0.136 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.347 | 0.278 | 3.60M | nan | — | — |
| 1,500 | 10 | 2.296 | 2.906 | 3.44M | nan | — | — |
| 1,500 | 100 | 10.999 | 10.024 | 9.98M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.08M | 5.90M | 1.00× | 1.18M | 1.07M | 1.00× | — |
| 2 | 10.58M | 9.41M | 1.59× | 1.29M | 1.43M | 1.33× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
