# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.11M | 0.003 | 285.99M | 0.035 | 5.27× | 9.90× |
| 10,000 | 0.074 | 134.69M | 0.068 | 146.98M | 0.132 | 1.77× | 1.94× |
| 100,000 | 0.916 | 109.19M | 0.884 | 113.15M | 1.100 | 1.20× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.052 | 0.128 | 2.48× |
| 1 | 5 | 0.288 | 0.489 | 1.70× |
| 1 | 10 | 0.395 | 0.981 | 2.48× |
| 10 | 1 | 0.045 | 0.086 | 1.90× |
| 10 | 5 | 0.189 | 0.433 | 2.28× |
| 10 | 10 | 0.408 | 0.944 | 2.31× |
| 100 | 1 | 0.059 | 0.105 | 1.79× |
| 100 | 5 | 0.203 | 0.457 | 2.25× |
| 100 | 10 | 0.397 | 0.956 | 2.41× |
| 1,000 | 1 | 0.055 | 0.112 | 2.04× |
| 1,000 | 5 | 0.199 | 0.504 | 2.54× |
| 1,000 | 10 | 0.481 | 1.033 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
