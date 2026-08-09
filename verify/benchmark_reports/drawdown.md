# Drawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.45M | 0.005 | 208.55M | nan | — | — |
| 10,000 | 0.041 | 243.84M | 0.038 | 260.64M | nan | — | — |
| 100,000 | 0.414 | 241.51M | 0.384 | 260.09M | nan | — | — |
| 1,000,000 | 4.286 | 233.33M | 3.842 | 260.26M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.408 ms**; native kernel **0.386 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.214 | 0.162 | 6.16M | nan | — | — |
| 100,000 | 10 | 0.842 | 0.498 | 20.08M | nan | — | — |
| 100,000 | 1,000 | 6.480 | 5.037 | 198.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 166.91M | 178.57M | 1.00× | 2.81M | 4.21M | 1.00× | — |
| 2 | 336.78M | 402.83M | 2.26× | 3.73M | 3.75M | 0.89× | — |
| 4 | 507.35M | 704.86M | 3.95× | 3.61M | 3.98M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
