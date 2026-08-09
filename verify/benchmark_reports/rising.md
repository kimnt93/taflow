# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.58M | 0.005 | 187.72M | nan | — | — |
| 10,000 | 0.045 | 221.89M | 0.042 | 237.39M | nan | — | — |
| 100,000 | 0.427 | 234.22M | 0.407 | 245.44M | nan | — | — |
| 1,000,000 | 4.569 | 218.85M | 4.115 | 243.03M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.432 ms**; native kernel **0.408 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.206 | 0.162 | 6.16M | nan | — | — |
| 100,000 | 10 | 1.007 | 0.620 | 16.12M | nan | — | — |
| 100,000 | 1,000 | 6.216 | 5.305 | 188.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 188.73M | 195.70M | 1.00× | 3.36M | 3.30M | 1.00× | — |
| 2 | 304.49M | 368.62M | 1.88× | 3.71M | 3.70M | 1.12× | — |
| 4 | 480.79M | 671.23M | 3.43× | 3.59M | 3.71M | 1.12× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
