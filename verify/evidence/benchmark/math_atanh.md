# MathAtanh benchmark (`numpy.arctanh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.70M | 0.013 | 75.59M | 0.026 | 1.98× | 2.00× |
| 10,000 | 0.119 | 83.88M | 0.116 | 86.03M | 0.148 | 1.24× | 1.27× |
| 100,000 | 1.150 | 86.96M | 1.180 | 84.72M | 1.395 | 1.21× | 1.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.091 | 1.14× |
| 1 | 5 | 0.298 | 0.280 | 0.94× |
| 1 | 10 | 0.481 | 0.601 | 1.25× |
| 10 | 1 | 0.052 | 0.059 | 1.13× |
| 10 | 5 | 0.215 | 0.300 | 1.40× |
| 10 | 10 | 0.501 | 0.568 | 1.13× |
| 100 | 1 | 0.050 | 0.058 | 1.18× |
| 100 | 5 | 0.220 | 0.272 | 1.24× |
| 100 | 10 | 0.487 | 0.596 | 1.22× |
| 1,000 | 1 | 0.065 | 0.073 | 1.12× |
| 1,000 | 5 | 0.229 | 0.306 | 1.34× |
| 1,000 | 10 | 0.487 | 0.779 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
