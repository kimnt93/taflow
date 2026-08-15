# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.18M | 0.016 | 61.72M | 0.042 | 2.20× | 2.60× |
| 10,000 | 0.172 | 58.01M | 0.169 | 59.29M | 0.187 | 1.08× | 1.11× |
| 100,000 | 1.785 | 56.01M | 1.665 | 60.07M | 1.567 | 0.88× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.129 | 1.87× |
| 1 | 5 | 0.264 | 0.447 | 1.69× |
| 1 | 10 | 0.396 | 0.969 | 2.45× |
| 10 | 1 | 0.043 | 0.099 | 2.31× |
| 10 | 5 | 0.199 | 0.439 | 2.20× |
| 10 | 10 | 0.380 | 0.909 | 2.39× |
| 100 | 1 | 0.044 | 0.094 | 2.15× |
| 100 | 5 | 0.186 | 0.504 | 2.70× |
| 100 | 10 | 0.424 | 0.962 | 2.27× |
| 1,000 | 1 | 0.061 | 0.102 | 1.68× |
| 1,000 | 5 | 0.206 | 0.510 | 2.47× |
| 1,000 | 10 | 0.449 | 1.157 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
