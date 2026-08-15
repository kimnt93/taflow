# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.97M | 0.015 | 65.78M | 0.036 | 2.01× | 2.36× |
| 10,000 | 0.150 | 66.87M | 0.149 | 66.92M | 0.119 | 0.80× | 0.80× |
| 100,000 | 1.480 | 67.55M | 1.461 | 68.44M | 0.885 | 0.60× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.145 | 1.73× |
| 1 | 5 | 0.302 | 0.541 | 1.79× |
| 1 | 10 | 0.402 | 0.898 | 2.23× |
| 10 | 1 | 0.049 | 0.086 | 1.77× |
| 10 | 5 | 0.189 | 0.417 | 2.20× |
| 10 | 10 | 0.410 | 0.933 | 2.27× |
| 100 | 1 | 0.045 | 0.088 | 1.96× |
| 100 | 5 | 0.192 | 0.446 | 2.32× |
| 100 | 10 | 0.397 | 0.963 | 2.42× |
| 1,000 | 1 | 0.058 | 0.104 | 1.78× |
| 1,000 | 5 | 0.206 | 0.486 | 2.36× |
| 1,000 | 10 | 0.429 | 1.017 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
