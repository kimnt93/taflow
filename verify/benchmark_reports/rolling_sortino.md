# RollingSortino benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.17M | 0.021 | 48.24M | nan | — | — |
| 10,000 | 0.194 | 51.52M | 0.190 | 52.76M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.032 ms**; native kernel **0.031 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.258 | 0.187 | 5.36M | nan | — | — |
| 1,500 | 10 | 1.161 | 0.719 | 13.91M | nan | — | — |
| 1,500 | 100 | 3.579 | 2.877 | 34.76M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.62M | 8.48M | 1.00× | 1.20M | 1.57M | 1.00× | — |
| 2 | 14.91M | 16.16M | 1.91× | 1.53M | 1.64M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
