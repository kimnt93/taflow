# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 202.45M | 0.004 | 249.30M | nan | — | — |
| 10,000 | 0.032 | 316.59M | 0.032 | 312.34M | nan | — | — |
| 100,000 | 0.311 | 321.58M | 0.285 | 350.44M | nan | — | — |
| 1,000,000 | 3.297 | 303.28M | 2.900 | 344.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.310 ms**; native kernel **0.287 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.214 | 0.158 | 6.32M | nan | — | — |
| 100,000 | 10 | 0.926 | 0.525 | 19.06M | nan | — | — |
| 100,000 | 1,000 | 4.932 | 4.122 | 242.59M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 242.16M | 316.51M | 1.00× | 3.93M | 4.19M | 1.00× | — |
| 2 | 444.76M | 558.28M | 1.76× | 4.02M | 3.80M | 0.91× | — |
| 4 | 404.52M | 460.11M | 1.45× | 3.80M | 3.89M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
