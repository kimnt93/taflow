# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.08M | 0.009 | 115.81M | nan | — | — |
| 10,000 | 0.081 | 123.17M | 0.076 | 131.69M | nan | — | — |
| 100,000 | 0.774 | 129.20M | 0.750 | 133.35M | nan | — | — |
| 1,000,000 | 8.509 | 117.52M | 7.802 | 128.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.816 ms**; native kernel **0.787 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.156 | 6.43M | nan | — | — |
| 100,000 | 10 | 0.680 | 0.480 | 20.81M | nan | — | — |
| 100,000 | 1,000 | 10.864 | 9.812 | 101.91M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.71M | 86.09M | 1.00× | 3.07M | 3.32M | 1.00× | — |
| 2 | 96.92M | 112.19M | 1.30× | 2.99M | 3.33M | 1.00× | — |
| 4 | 100.41M | 110.73M | 1.29× | 2.92M | 3.48M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
