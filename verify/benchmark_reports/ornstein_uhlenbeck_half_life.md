# OrnsteinUhlenbeckHalfLife benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.13M | 0.048 | 20.87M | nan | — | — |
| 10,000 | 0.442 | 22.62M | 0.458 | 21.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.069 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.270 | 0.205 | 4.88M | nan | — | — |
| 1,500 | 10 | 1.424 | 0.974 | 10.27M | nan | — | — |
| 1,500 | 100 | 6.188 | 5.664 | 17.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.81M | 6.21M | 1.00× | 1.30M | 1.14M | 1.00× | — |
| 2 | 13.64M | 15.57M | 2.51× | 1.43M | 1.56M | 1.37× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
