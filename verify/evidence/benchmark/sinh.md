# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.04M | 0.010 | 101.45M | 0.036 | 3.61× | 3.70× |
| 10,000 | 0.073 | 136.65M | 0.069 | 145.30M | 0.101 | 1.38× | 1.47× |
| 100,000 | 0.767 | 130.34M | 0.686 | 145.85M | 0.736 | 0.96× | 1.07× |
| 1,000,000 | 7.384 | 135.42M | 6.596 | 151.61M | 7.043 | 0.95× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.147 | 2.02× |
| 1 | 5 | 0.283 | 0.501 | 1.77× |
| 1 | 10 | 0.511 | 0.957 | 1.87× |
| 10 | 1 | 0.049 | 0.112 | 2.30× |
| 10 | 5 | 0.318 | 0.582 | 1.83× |
| 10 | 10 | 0.518 | 1.002 | 1.93× |
| 100 | 1 | 0.052 | 0.086 | 1.67× |
| 100 | 5 | 0.246 | 0.479 | 1.95× |
| 100 | 10 | 0.590 | 0.950 | 1.61× |
| 1,000 | 1 | 0.075 | 0.120 | 1.61× |
| 1,000 | 5 | 0.286 | 0.514 | 1.79× |
| 1,000 | 10 | 0.606 | 1.040 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
