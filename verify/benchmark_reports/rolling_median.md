# RollingMedian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 36.29M | 0.026 | 38.89M | nan | — | — |
| 10,000 | 0.311 | 32.19M | 0.315 | 31.75M | nan | — | — |
| 100,000 | 3.085 | 32.42M | 3.117 | 32.08M | nan | — | — |
| 1,000,000 | 31.342 | 31.91M | 31.053 | 32.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.078 ms**; native kernel **3.081 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.174 | 5.73M | nan | — | — |
| 100,000 | 10 | 1.328 | 0.977 | 10.24M | nan | — | — |
| 100,000 | 1,000 | 37.305 | 31.125 | 32.13M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 29.51M | 30.05M | 1.00× | 3.33M | 3.64M | 1.00× | — |
| 2 | 54.14M | 56.55M | 1.88× | 2.93M | 3.19M | 0.87× | — |
| 4 | 103.61M | 102.74M | 3.42× | 2.88M | 3.05M | 0.84× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
