# RollingCov benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.57M | 0.039 | 25.59M | nan | — | — |
| 10,000 | 0.346 | 28.92M | 0.328 | 30.46M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.058 ms**; native kernel **0.057 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.305 | 0.230 | 4.35M | nan | — | — |
| 1,500 | 10 | 2.720 | 1.071 | 9.34M | nan | — | — |
| 1,500 | 100 | 5.350 | 4.584 | 21.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.92M | 10.45M | 1.00× | 1.28M | 1.27M | 1.00× | — |
| 2 | 14.41M | 15.25M | 1.46× | 1.18M | 1.44M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
