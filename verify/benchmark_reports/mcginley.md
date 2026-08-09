# McGinleyDynamic benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.60M | 0.016 | 60.69M | nan | — | — |
| 10,000 | 0.145 | 68.85M | 0.144 | 69.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.024 ms**; native kernel **0.023 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.232 | 0.164 | 6.10M | nan | — | — |
| 1,500 | 10 | 1.051 | 0.632 | 15.81M | nan | — | — |
| 1,500 | 100 | 2.967 | 2.459 | 40.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.31M | 16.13M | 1.00× | 1.61M | 1.57M | 1.00× | — |
| 2 | 18.46M | 21.35M | 1.32× | 1.37M | 1.64M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
