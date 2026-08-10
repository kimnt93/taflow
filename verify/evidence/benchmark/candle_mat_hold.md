# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.46M | 0.018 | 55.76M | 0.037 | 1.84× | 2.08× |
| 10,000 | 0.165 | 60.62M | 0.163 | 61.39M | 0.113 | 0.68× | 0.69× |
| 100,000 | 1.776 | 56.32M | 1.615 | 61.93M | 0.824 | 0.46× | 0.51× |
| 1,000,000 | 16.598 | 60.25M | 16.869 | 59.28M | 8.480 | 0.51× | 0.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.126 | 1.06× |
| 1 | 5 | 0.323 | 0.500 | 1.55× |
| 1 | 10 | 0.536 | 0.944 | 1.76× |
| 10 | 1 | 0.058 | 0.092 | 1.59× |
| 10 | 5 | 0.249 | 0.455 | 1.83× |
| 10 | 10 | 0.534 | 0.954 | 1.79× |
| 100 | 1 | 0.065 | 0.090 | 1.37× |
| 100 | 5 | 0.262 | 0.456 | 1.74× |
| 100 | 10 | 0.551 | 0.938 | 1.70× |
| 1,000 | 1 | 0.074 | 0.107 | 1.44× |
| 1,000 | 5 | 0.270 | 0.503 | 1.87× |
| 1,000 | 10 | 0.560 | 1.049 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
