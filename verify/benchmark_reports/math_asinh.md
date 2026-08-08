# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.44M | 0.013 | 78.25M | nan | — | — |
| 10,000 | 0.118 | 84.41M | 0.116 | 86.50M | nan | — | — |
| 100,000 | 1.165 | 85.81M | 1.141 | 87.63M | nan | — | — |
| 1,000,000 | 12.263 | 81.55M | 12.408 | 80.59M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.143 ms**; native kernel **1.114 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.164 | 6.09M | nan | — | — |
| 100,000 | 10 | 1.037 | 0.598 | 16.72M | nan | — | — |
| 100,000 | 1,000 | 13.425 | 12.627 | 79.20M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.58M | 74.14M | 1.00× | 2.78M | 2.77M | 1.00× | — |
| 2 | 129.59M | 148.98M | 2.01× | 2.73M | 3.10M | 1.12× | — |
| 4 | 212.83M | 273.91M | 3.69× | 2.71M | 3.15M | 1.14× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
