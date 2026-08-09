# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.01M | 0.020 | 50.76M | nan | — | — |
| 10,000 | 0.319 | 31.36M | 0.268 | 37.32M | nan | — | — |
| 100,000 | 3.170 | 31.55M | 2.573 | 38.86M | nan | — | — |
| 1,000,000 | 58.515 | 17.09M | 42.903 | 23.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.209 ms**; native kernel **2.571 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.284 | 0.229 | 4.37M | nan | — | — |
| 100,000 | 10 | 0.824 | 0.632 | 15.82M | nan | — | — |
| 100,000 | 1,000 | 31.558 | 27.088 | 36.92M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 29.47M | 34.50M | 1.00× | 1.76M | 2.41M | 1.00× | — |
| 2 | 28.65M | 33.21M | 0.96× | 1.95M | 2.36M | 0.98× | — |
| 4 | 29.68M | 32.59M | 0.94× | 1.96M | 2.21M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
