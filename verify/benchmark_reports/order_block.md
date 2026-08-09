# OrderBlock benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.078 | 12.74M | 0.074 | 13.51M | nan | — | — |
| 10,000 | 0.863 | 11.59M | 0.829 | 12.07M | nan | — | — |
| 100,000 | 9.367 | 10.68M | 8.636 | 11.58M | nan | — | — |
| 1,000,000 | 104.412 | 9.58M | 85.827 | 11.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **8.909 ms**; native kernel **8.581 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.467 | 0.398 | 2.51M | nan | — | — |
| 100,000 | 10 | 3.256 | 2.103 | 4.75M | nan | — | — |
| 100,000 | 1,000 | 98.422 | 90.569 | 11.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.29M | 11.17M | 1.00× | 1.52M | 1.44M | 1.00× | — |
| 2 | 19.49M | 21.50M | 1.92× | 1.61M | 1.64M | 1.15× | — |
| 4 | 34.01M | 32.86M | 2.94× | 1.64M | 1.65M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
