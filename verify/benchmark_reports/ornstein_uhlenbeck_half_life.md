# OrnsteinUhlenbeckHalfLife benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.43M | 0.051 | 19.75M | nan | — | — |
| 10,000 | 0.577 | 17.33M | 0.471 | 21.25M | nan | — | — |
| 100,000 | 4.799 | 20.84M | 4.626 | 21.62M | nan | — | — |
| 1,000,000 | 48.762 | 20.51M | 47.346 | 21.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.721 ms**; native kernel **4.633 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.204 | 4.89M | nan | — | — |
| 100,000 | 10 | 1.354 | 0.934 | 10.71M | nan | — | — |
| 100,000 | 1,000 | 49.792 | 50.956 | 19.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 20.70M | 20.82M | 1.00× | 2.65M | 3.05M | 1.00× | — |
| 2 | 37.96M | 39.83M | 1.91× | 2.70M | 2.55M | 0.84× | — |
| 4 | 50.32M | 54.86M | 2.63× | 2.65M | 2.81M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
