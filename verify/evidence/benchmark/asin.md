# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.89M | 0.008 | 118.98M | 0.033 | 3.67× | 3.91× |
| 10,000 | 0.066 | 150.62M | 0.061 | 163.03M | 0.090 | 1.36× | 1.47× |
| 100,000 | 0.637 | 156.92M | 0.618 | 161.93M | 0.638 | 1.00× | 1.03× |
| 1,000,000 | 7.284 | 137.28M | 6.316 | 158.33M | 6.144 | 0.84× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.179 | 0.160 | 0.89× |
| 1 | 5 | 0.259 | 0.436 | 1.69× |
| 1 | 10 | 0.471 | 0.879 | 1.86× |
| 10 | 1 | 0.056 | 0.090 | 1.60× |
| 10 | 5 | 0.215 | 0.416 | 1.93× |
| 10 | 10 | 0.453 | 0.897 | 1.98× |
| 100 | 1 | 0.047 | 0.083 | 1.76× |
| 100 | 5 | 0.219 | 0.416 | 1.90× |
| 100 | 10 | 0.489 | 0.949 | 1.94× |
| 1,000 | 1 | 0.061 | 0.094 | 1.54× |
| 1,000 | 5 | 0.233 | 0.437 | 1.88× |
| 1,000 | 10 | 0.503 | 0.941 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
