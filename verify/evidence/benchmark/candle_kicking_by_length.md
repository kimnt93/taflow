# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.94M | 0.018 | 54.68M | 0.039 | 1.69× | 2.15× |
| 10,000 | 0.178 | 56.32M | 0.157 | 63.53M | 0.184 | 1.03× | 1.17× |
| 100,000 | 1.985 | 50.37M | 1.883 | 53.12M | 1.547 | 0.78× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.134 | 1.26× |
| 1 | 5 | 0.301 | 0.460 | 1.53× |
| 1 | 10 | 0.578 | 0.951 | 1.65× |
| 10 | 1 | 0.059 | 0.099 | 1.69× |
| 10 | 5 | 0.259 | 0.452 | 1.75× |
| 10 | 10 | 0.576 | 0.963 | 1.67× |
| 100 | 1 | 0.063 | 0.088 | 1.40× |
| 100 | 5 | 0.328 | 0.480 | 1.46× |
| 100 | 10 | 0.606 | 1.086 | 1.79× |
| 1,000 | 1 | 0.100 | 0.123 | 1.23× |
| 1,000 | 5 | 0.301 | 0.575 | 1.91× |
| 1,000 | 10 | 0.636 | 1.177 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
