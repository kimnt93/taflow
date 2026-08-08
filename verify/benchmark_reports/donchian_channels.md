# Donchian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 32.80M | 0.027 | 37.35M | nan | — | — |
| 10,000 | 0.315 | 31.72M | 0.306 | 32.68M | nan | — | — |
| 100,000 | 3.775 | 26.49M | 3.732 | 26.79M | nan | — | — |
| 1,000,000 | 42.125 | 23.74M | 31.882 | 31.37M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.115 ms**; native kernel **3.110 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.273 | 3.67M | nan | — | — |
| 100,000 | 10 | 2.307 | 1.169 | 8.55M | nan | — | — |
| 100,000 | 1,000 | 33.161 | 32.986 | 30.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 28.56M | 31.90M | 1.00× | 2.24M | 2.39M | 1.00× | — |
| 2 | 44.04M | 48.30M | 1.51× | 2.24M | 2.29M | 0.96× | — |
| 4 | 74.65M | 89.80M | 2.82× | 2.28M | 2.28M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
