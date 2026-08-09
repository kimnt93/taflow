# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.58M | 0.007 | 142.63M | 0.032 | 4.08× | 4.63× |
| 10,000 | 0.065 | 153.81M | 0.064 | 155.05M | 0.082 | 1.26× | 1.27× |
| 100,000 | 0.632 | 158.19M | 0.610 | 164.05M | 0.569 | 0.90× | 0.93× |
| 1,000,000 | 6.974 | 143.39M | 6.840 | 146.20M | 5.455 | 0.78× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.632 ms**; native kernel **0.609 ms**; TA-Lib 0.571 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.162 | 6.17M | 571.600 | 3523.95× | 175.90× |
| 100,000 | 10 | 1.078 | 0.565 | 17.71M | 562.975 | 996.77× | 46.19× |
| 100,000 | 1,000 | 8.672 | 9.308 | 107.44M | 567.902 | 61.01× | 3.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 121.52M | 121.66M | 1.00× | 2.41M | 3.00M | 1.00× | 143.86M |
| 2 | 234.28M | 219.13M | 1.80× | 3.09M | 3.47M | 1.16× | 138.19M |
| 4 | 297.69M | 390.24M | 3.21× | 3.14M | 2.92M | 0.98× | 142.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
