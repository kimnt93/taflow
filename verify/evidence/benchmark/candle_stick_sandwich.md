# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.04M | 0.046 | 21.76M | 0.032 | 0.57× | 0.69× |
| 10,000 | 0.322 | 31.05M | 0.405 | 24.68M | 0.112 | 0.35× | 0.28× |
| 100,000 | 3.183 | 31.42M | 2.966 | 33.71M | 0.601 | 0.19× | 0.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.129 | 0.81× |
| 1 | 5 | 0.463 | 0.490 | 1.06× |
| 1 | 10 | 0.644 | 0.894 | 1.39× |
| 10 | 1 | 0.069 | 0.092 | 1.33× |
| 10 | 5 | 0.319 | 0.430 | 1.35× |
| 10 | 10 | 0.634 | 0.974 | 1.54× |
| 100 | 1 | 0.080 | 0.089 | 1.12× |
| 100 | 5 | 0.324 | 0.435 | 1.34× |
| 100 | 10 | 0.665 | 0.905 | 1.36× |
| 1,000 | 1 | 0.104 | 0.099 | 0.95× |
| 1,000 | 5 | 0.332 | 0.458 | 1.38× |
| 1,000 | 10 | 0.684 | 1.005 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
