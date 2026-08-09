# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.00M | 0.012 | 82.82M | nan | — | — |
| 10,000 | 0.114 | 87.42M | 0.111 | 89.99M | nan | — | — |
| 100,000 | 1.109 | 90.17M | 1.107 | 90.37M | nan | — | — |
| 1,000,000 | 12.054 | 82.96M | 11.504 | 86.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.158 ms**; native kernel **1.084 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.163 | 6.15M | nan | — | — |
| 100,000 | 10 | 0.984 | 0.595 | 16.81M | nan | — | — |
| 100,000 | 1,000 | 13.185 | 12.498 | 80.01M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 70.32M | 83.75M | 1.00× | 3.06M | 3.02M | 1.00× | — |
| 2 | 141.99M | 160.13M | 1.91× | 2.99M | 3.31M | 1.10× | — |
| 4 | 199.98M | 265.50M | 3.17× | 2.92M | 2.88M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
