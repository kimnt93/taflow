# RollingKurtosis benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.57M | 0.030 | 33.37M | nan | — | — |
| 10,000 | 0.279 | 35.80M | 0.288 | 34.70M | nan | — | — |
| 100,000 | 2.880 | 34.72M | 2.754 | 36.31M | nan | — | — |
| 1,000,000 | 29.296 | 34.13M | 28.193 | 35.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.900 ms**; native kernel **2.920 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.306 | 0.195 | 5.13M | nan | — | — |
| 100,000 | 10 | 1.415 | 0.908 | 11.01M | nan | — | — |
| 100,000 | 1,000 | 32.822 | 31.338 | 31.91M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 34.93M | 35.55M | 1.00× | 3.47M | 3.84M | 1.00× | — |
| 2 | 62.24M | 64.24M | 1.81× | 2.97M | 3.43M | 0.89× | — |
| 4 | 81.13M | 89.28M | 2.51× | 2.90M | 3.26M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
