# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.92M | 0.008 | 122.39M | 0.032 | 2.96× | 3.95× |
| 10,000 | 0.060 | 165.56M | 0.060 | 165.82M | 0.080 | 1.32× | 1.32× |
| 100,000 | 0.650 | 153.90M | 0.615 | 162.48M | 0.523 | 0.80× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.112 | 0.62× |
| 1 | 5 | 0.274 | 0.507 | 1.85× |
| 1 | 10 | 0.565 | 0.967 | 1.71× |
| 10 | 1 | 0.060 | 0.090 | 1.50× |
| 10 | 5 | 0.277 | 0.462 | 1.67× |
| 10 | 10 | 0.611 | 1.051 | 1.72× |
| 100 | 1 | 0.064 | 0.098 | 1.53× |
| 100 | 5 | 0.264 | 0.469 | 1.78× |
| 100 | 10 | 0.550 | 1.003 | 1.82× |
| 1,000 | 1 | 0.062 | 0.102 | 1.65× |
| 1,000 | 5 | 0.271 | 0.456 | 1.68× |
| 1,000 | 10 | 0.567 | 0.987 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
