# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.47M | 0.003 | 300.62M | 0.034 | 5.10× | 10.32× |
| 10,000 | 0.074 | 134.99M | 0.069 | 145.35M | 0.135 | 1.82× | 1.96× |
| 100,000 | 0.917 | 109.04M | 0.872 | 114.64M | 1.090 | 1.19× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.108 | 1.89× |
| 1 | 5 | 0.319 | 0.456 | 1.43× |
| 1 | 10 | 0.423 | 0.909 | 2.15× |
| 10 | 1 | 0.041 | 0.089 | 2.18× |
| 10 | 5 | 0.180 | 0.423 | 2.35× |
| 10 | 10 | 0.383 | 0.985 | 2.57× |
| 100 | 1 | 0.061 | 0.113 | 1.85× |
| 100 | 5 | 0.240 | 0.450 | 1.87× |
| 100 | 10 | 0.387 | 0.933 | 2.41× |
| 1,000 | 1 | 0.054 | 0.107 | 1.98× |
| 1,000 | 5 | 0.223 | 0.566 | 2.54× |
| 1,000 | 10 | 0.432 | 1.023 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
