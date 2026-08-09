# Supertrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.95M | 0.017 | 60.06M | nan | — | — |
| 10,000 | 0.180 | 55.44M | 0.163 | 61.20M | nan | — | — |
| 100,000 | 1.807 | 55.34M | 1.634 | 61.20M | nan | — | — |
| 1,000,000 | 37.223 | 26.87M | 25.319 | 39.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.795 ms**; native kernel **1.610 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.350 | 0.275 | 3.64M | nan | — | — |
| 100,000 | 10 | 2.018 | 1.113 | 8.99M | nan | — | — |
| 100,000 | 1,000 | 19.497 | 17.094 | 58.50M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.62M | 54.01M | 1.00× | 1.94M | 2.02M | 1.00× | — |
| 2 | 79.14M | 95.92M | 1.78× | 2.12M | 2.12M | 1.05× | — |
| 4 | 85.84M | 111.33M | 2.06× | 2.13M | 2.01M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
