# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.79M | 0.012 | 81.39M | 0.046 | 2.54× | 3.78× |
| 10,000 | 0.149 | 67.22M | 0.144 | 69.68M | 0.211 | 1.42× | 1.47× |
| 100,000 | 1.503 | 66.53M | 1.501 | 66.62M | 1.741 | 1.16× | 1.16× |
| 1,000,000 | 14.867 | 67.26M | 14.799 | 67.57M | 18.271 | 1.23× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.128 | 1.64× |
| 1 | 5 | 0.279 | 0.455 | 1.63× |
| 1 | 10 | 0.702 | 1.084 | 1.54× |
| 10 | 1 | 0.059 | 0.093 | 1.56× |
| 10 | 5 | 0.290 | 0.459 | 1.58× |
| 10 | 10 | 0.576 | 1.119 | 1.94× |
| 100 | 1 | 0.079 | 0.103 | 1.30× |
| 100 | 5 | 0.336 | 0.467 | 1.39× |
| 100 | 10 | 0.564 | 1.134 | 2.01× |
| 1,000 | 1 | 0.094 | 0.157 | 1.67× |
| 1,000 | 5 | 0.336 | 0.541 | 1.61× |
| 1,000 | 10 | 0.557 | 1.225 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
