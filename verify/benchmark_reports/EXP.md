# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.81M | 0.007 | 138.51M | 0.030 | 3.29× | 4.12× |
| 10,000 | 0.054 | 183.67M | 0.051 | 195.51M | 0.069 | 1.27× | 1.36× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.297 | 0.168 | 5.95M | 32.220 | 191.76× | 146.48× |
| 1,500 | 10 | 1.151 | 0.760 | 13.16M | 32.638 | 42.96× | 35.36× |
| 1,500 | 100 | 3.418 | 2.310 | 43.29M | 35.215 | 15.24× | 11.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.47M | 14.92M | 1.00× | 1.25M | 1.50M | 1.00× | 9.46M |
| 2 | 14.30M | 16.89M | 1.13× | 1.06M | 1.29M | 0.85× | 8.43M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
