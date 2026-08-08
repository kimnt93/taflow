# RollingCov benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.35M | 0.021 | 47.16M | nan | — | — |
| 10,000 | 0.202 | 49.41M | 0.197 | 50.88M | nan | — | — |
| 100,000 | 1.994 | 50.14M | 1.936 | 51.65M | nan | — | — |
| 1,000,000 | 20.662 | 48.40M | 20.798 | 48.08M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.015 ms**; native kernel **1.946 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.287 | 0.216 | 4.62M | nan | — | — |
| 100,000 | 10 | 1.913 | 0.957 | 10.45M | nan | — | — |
| 100,000 | 1,000 | 22.800 | 23.465 | 42.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.22M | 44.76M | 1.00× | 2.97M | 2.66M | 1.00× | — |
| 2 | 88.99M | 93.80M | 2.10× | 2.88M | 2.93M | 1.10× | — |
| 4 | 163.59M | 190.76M | 4.26× | 2.46M | 2.60M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
