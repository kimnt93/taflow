# RollingCov benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.27M | 0.019 | 52.56M | nan | — | — |
| 10,000 | 0.175 | 57.03M | 0.173 | 57.93M | nan | — | — |
| 100,000 | 1.775 | 56.33M | 1.731 | 57.77M | nan | — | — |
| 1,000,000 | 18.220 | 54.88M | 18.015 | 55.51M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.943 ms**; native kernel **1.762 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.254 | 0.202 | 4.95M | nan | — | — |
| 100,000 | 10 | 1.540 | 1.013 | 9.87M | nan | — | — |
| 100,000 | 1,000 | 19.999 | 18.812 | 53.16M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 47.32M | 51.59M | 1.00× | 2.32M | 2.85M | 1.00× | — |
| 2 | 87.54M | 97.80M | 1.90× | 2.73M | 2.89M | 1.02× | — |
| 4 | 112.70M | 112.15M | 2.17× | 2.65M | 2.71M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
