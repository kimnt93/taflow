# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.71M | 0.008 | 126.67M | 0.035 | 4.68× | 4.40× |
| 10,000 | 0.046 | 217.34M | 0.045 | 221.28M | 0.064 | 1.40× | 1.43× |
| 100,000 | 0.441 | 226.62M | 0.425 | 235.48M | 0.336 | 0.76× | 0.79× |
| 1,000,000 | 4.923 | 203.14M | 4.471 | 223.66M | 3.071 | 0.62× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.111 | 1.50× |
| 1 | 5 | 0.306 | 0.461 | 1.51× |
| 1 | 10 | 0.551 | 1.059 | 1.92× |
| 10 | 1 | 0.049 | 0.095 | 1.91× |
| 10 | 5 | 0.277 | 0.479 | 1.73× |
| 10 | 10 | 0.469 | 0.983 | 2.10× |
| 100 | 1 | 0.053 | 0.096 | 1.83× |
| 100 | 5 | 0.226 | 0.455 | 2.01× |
| 100 | 10 | 0.454 | 0.928 | 2.05× |
| 1,000 | 1 | 0.058 | 0.109 | 1.87× |
| 1,000 | 5 | 0.266 | 0.513 | 1.93× |
| 1,000 | 10 | 0.472 | 0.993 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
