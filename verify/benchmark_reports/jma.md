# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.99M | 0.021 | 47.00M | nan | — | — |
| 10,000 | 0.202 | 49.53M | 0.195 | 51.38M | nan | — | — |
| 100,000 | 1.971 | 50.73M | 1.965 | 50.88M | nan | — | — |
| 1,000,000 | 20.237 | 49.41M | 19.854 | 50.37M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.972 ms**; native kernel **2.139 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.179 | 5.58M | nan | — | — |
| 100,000 | 10 | 0.848 | 0.642 | 15.57M | nan | — | — |
| 100,000 | 1,000 | 21.437 | 21.588 | 46.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 40.50M | 49.55M | 1.00× | 3.08M | 2.91M | 1.00× | — |
| 2 | 44.38M | 48.31M | 0.98× | 2.27M | 3.29M | 1.13× | — |
| 4 | 45.27M | 46.04M | 0.93× | 2.64M | 3.30M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
