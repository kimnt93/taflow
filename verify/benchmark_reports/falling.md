# Falling benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.88M | 0.005 | 186.21M | nan | — | — |
| 10,000 | 0.045 | 222.69M | 0.044 | 228.85M | nan | — | — |
| 100,000 | 0.427 | 234.16M | 0.403 | 248.10M | nan | — | — |
| 1,000,000 | 4.539 | 220.32M | 4.314 | 231.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.446 ms**; native kernel **0.415 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.216 | 0.147 | 6.80M | nan | — | — |
| 100,000 | 10 | 0.903 | 0.517 | 19.35M | nan | — | — |
| 100,000 | 1,000 | 5.995 | 5.442 | 183.75M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 165.14M | 201.98M | 1.00× | 3.53M | 3.49M | 1.00× | — |
| 2 | 320.36M | 385.56M | 1.91× | 3.56M | 3.83M | 1.10× | — |
| 4 | 467.88M | 687.95M | 3.41× | 3.82M | 3.96M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
