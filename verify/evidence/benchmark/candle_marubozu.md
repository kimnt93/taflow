# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.38M | 0.016 | 62.50M | 0.035 | 1.79× | 2.18× |
| 10,000 | 0.146 | 68.34M | 0.140 | 71.52M | 0.135 | 0.92× | 0.96× |
| 100,000 | 1.411 | 70.90M | 1.573 | 63.57M | 1.156 | 0.82× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.138 | 1.20× |
| 1 | 5 | 0.321 | 0.447 | 1.39× |
| 1 | 10 | 0.584 | 0.954 | 1.63× |
| 10 | 1 | 0.060 | 0.096 | 1.59× |
| 10 | 5 | 0.294 | 0.473 | 1.61× |
| 10 | 10 | 0.578 | 0.929 | 1.61× |
| 100 | 1 | 0.063 | 0.092 | 1.45× |
| 100 | 5 | 0.261 | 0.503 | 1.93× |
| 100 | 10 | 0.599 | 0.963 | 1.61× |
| 1,000 | 1 | 0.068 | 0.138 | 2.05× |
| 1,000 | 5 | 0.284 | 0.478 | 1.68× |
| 1,000 | 10 | 0.642 | 1.059 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
