# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.66M | 0.011 | 92.19M | 0.038 | 2.98× | 3.54× |
| 10,000 | 0.100 | 100.15M | 0.092 | 109.10M | 0.114 | 1.14× | 1.25× |
| 100,000 | 0.921 | 108.64M | 0.880 | 113.66M | 0.868 | 0.94× | 0.99× |
| 1,000,000 | 9.603 | 104.13M | 8.585 | 116.48M | 8.257 | 0.86× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.107 | 1.33× |
| 1 | 5 | 0.349 | 0.434 | 1.25× |
| 1 | 10 | 0.455 | 0.952 | 2.09× |
| 10 | 1 | 0.056 | 0.088 | 1.57× |
| 10 | 5 | 0.296 | 0.480 | 1.62× |
| 10 | 10 | 0.472 | 0.875 | 1.85× |
| 100 | 1 | 0.048 | 0.091 | 1.91× |
| 100 | 5 | 0.278 | 0.489 | 1.76× |
| 100 | 10 | 0.513 | 0.938 | 1.83× |
| 1,000 | 1 | 0.058 | 0.100 | 1.72× |
| 1,000 | 5 | 0.285 | 0.574 | 2.01× |
| 1,000 | 10 | 0.576 | 1.026 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
