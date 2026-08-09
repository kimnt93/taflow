# LowerLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.97M | 0.006 | 161.07M | nan | — | — |
| 10,000 | 0.037 | 273.28M | 0.034 | 294.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.357 | 0.199 | 5.03M | nan | — | — |
| 1,500 | 10 | 1.565 | 0.747 | 13.39M | nan | — | — |
| 1,500 | 100 | 2.614 | 1.665 | 60.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.64M | 17.16M | 1.00× | 1.05M | 1.49M | 1.00× | — |
| 2 | 18.58M | 21.50M | 1.25× | 1.25M | 1.46M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
