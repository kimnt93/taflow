# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.80M | 0.017 | 60.35M | 0.039 | 2.01× | 2.34× |
| 10,000 | 0.158 | 63.16M | 0.154 | 65.06M | 0.122 | 0.77× | 0.79× |
| 100,000 | 1.570 | 63.70M | 1.602 | 62.43M | 0.885 | 0.56× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.111 | 0.87× |
| 1 | 5 | 0.407 | 0.481 | 1.18× |
| 1 | 10 | 0.553 | 0.992 | 1.79× |
| 10 | 1 | 0.055 | 0.097 | 1.77× |
| 10 | 5 | 0.264 | 0.469 | 1.78× |
| 10 | 10 | 0.546 | 1.020 | 1.87× |
| 100 | 1 | 0.072 | 0.098 | 1.37× |
| 100 | 5 | 0.291 | 0.481 | 1.65× |
| 100 | 10 | 0.542 | 0.981 | 1.81× |
| 1,000 | 1 | 0.069 | 0.102 | 1.48× |
| 1,000 | 5 | 0.298 | 0.520 | 1.74× |
| 1,000 | 10 | 0.555 | 1.059 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
