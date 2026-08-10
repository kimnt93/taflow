# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.44M | 0.015 | 65.25M | 0.043 | 2.58× | 2.83× |
| 10,000 | 0.155 | 64.54M | 0.108 | 92.47M | 0.115 | 0.74× | 1.06× |
| 100,000 | 1.113 | 89.87M | 1.346 | 74.28M | 0.780 | 0.70× | 0.58× |
| 1,000,000 | 11.192 | 89.35M | 11.686 | 85.58M | 7.735 | 0.69× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.135 | 1.97× |
| 1 | 5 | 0.350 | 0.678 | 1.94× |
| 1 | 10 | 0.588 | 1.043 | 1.77× |
| 10 | 1 | 0.055 | 0.089 | 1.61× |
| 10 | 5 | 0.237 | 0.533 | 2.24× |
| 10 | 10 | 0.889 | 1.111 | 1.25× |
| 100 | 1 | 0.093 | 0.105 | 1.14× |
| 100 | 5 | 0.262 | 0.610 | 2.33× |
| 100 | 10 | 0.661 | 1.123 | 1.70× |
| 1,000 | 1 | 0.074 | 0.121 | 1.64× |
| 1,000 | 5 | 0.271 | 0.526 | 1.94× |
| 1,000 | 10 | 0.630 | 1.296 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
