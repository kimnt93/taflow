# ExponentiallyWeightedCorrelation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.96M | 0.014 | 70.35M | nan | — | — |
| 10,000 | 0.079 | 127.17M | 0.073 | 137.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.271 | 0.197 | 5.07M | nan | — | — |
| 1,500 | 10 | 1.543 | 0.818 | 12.23M | nan | — | — |
| 1,500 | 100 | 2.735 | 2.095 | 47.73M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 14.88M | 13.97M | 1.00× | 1.36M | 1.53M | 1.00× | — |
| 2 | 17.28M | 21.81M | 1.56× | 1.45M | 1.30M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
