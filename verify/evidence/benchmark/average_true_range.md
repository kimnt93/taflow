# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.20M | 0.013 | 77.60M | 0.049 | 3.94× | 3.81× |
| 10,000 | 0.084 | 118.84M | 0.074 | 135.00M | 0.094 | 1.11× | 1.27× |
| 100,000 | 0.679 | 147.35M | 0.657 | 152.12M | 0.644 | 0.95× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.132 | 1.24× |
| 1 | 5 | 0.270 | 0.471 | 1.74× |
| 1 | 10 | 0.501 | 0.986 | 1.97× |
| 10 | 1 | 0.051 | 0.089 | 1.75× |
| 10 | 5 | 0.240 | 0.464 | 1.94× |
| 10 | 10 | 0.530 | 1.076 | 2.03× |
| 100 | 1 | 0.065 | 0.108 | 1.65× |
| 100 | 5 | 0.252 | 0.470 | 1.87× |
| 100 | 10 | 0.491 | 0.992 | 2.02× |
| 1,000 | 1 | 0.097 | 0.103 | 1.06× |
| 1,000 | 5 | 0.271 | 0.554 | 2.05× |
| 1,000 | 10 | 0.540 | 1.048 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
