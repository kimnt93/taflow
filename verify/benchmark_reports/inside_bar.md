# InsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 205.27M | 0.004 | 273.72M | nan | — | — |
| 10,000 | 0.029 | 344.35M | 0.026 | 386.84M | nan | — | — |
| 100,000 | 0.271 | 368.71M | 0.248 | 403.51M | nan | — | — |
| 1,000,000 | 3.234 | 309.21M | 2.762 | 362.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.273 ms**; native kernel **0.246 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.173 | 5.77M | nan | — | — |
| 100,000 | 10 | 1.424 | 0.755 | 13.24M | nan | — | — |
| 100,000 | 1,000 | 4.973 | 7.408 | 134.98M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 230.33M | 248.04M | 1.00× | 2.49M | 3.44M | 1.00× | — |
| 2 | 421.43M | 532.22M | 2.15× | 3.25M | 3.25M | 0.94× | — |
| 4 | 609.05M | 765.90M | 3.09× | 3.41M | 3.50M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
