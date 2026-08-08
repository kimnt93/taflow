# RollingZScore benchmark

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.05M | 0.020 | 51.11M | nan | — | — |
| 10,000 | 0.187 | 53.36M | 0.189 | 52.86M | nan | — | — |
| 100,000 | 1.902 | 52.57M | 1.820 | 54.96M | nan | — | — |
| 1,000,000 | 20.364 | 49.11M | 18.656 | 53.60M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.189 ms**; native kernel **1.856 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.251 | 0.186 | 5.37M | nan | — | — |
| 100,000 | 10 | 1.049 | 0.680 | 14.70M | nan | — | — |
| 100,000 | 1,000 | 29.877 | 19.381 | 51.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 47.91M | 52.13M | 1.00× | 3.21M | 2.49M | 1.00× | — |
| 2 | 77.14M | 84.06M | 1.61× | 2.39M | 3.10M | 1.24× | — |
| 4 | 129.57M | 171.20M | 3.28× | 2.86M | 2.77M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
