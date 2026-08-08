# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.095 | 10.53M | 0.096 | 10.44M | nan | — | — |
| 10,000 | 0.956 | 10.46M | 0.960 | 10.42M | nan | — | — |
| 100,000 | 9.519 | 10.51M | 9.900 | 10.10M | nan | — | — |
| 1,000,000 | 95.110 | 10.51M | 92.964 | 10.76M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.919 ms**; native kernel **9.492 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.379 | 0.345 | 2.90M | nan | — | — |
| 100,000 | 10 | 2.545 | 1.968 | 5.08M | nan | — | — |
| 100,000 | 1,000 | 98.533 | 113.411 | 8.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.52M | 10.34M | 1.00× | 2.10M | 2.25M | 1.00× | — |
| 2 | 19.88M | 19.76M | 1.91× | 2.14M | 2.37M | 1.05× | — |
| 4 | 37.49M | 40.35M | 3.90× | 2.10M | 2.18M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
