# MathAsinh benchmark (`numpy.arcsinh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.89M | 0.032 | 30.97M | 0.024 | 0.63× | 0.73× |
| 10,000 | 0.304 | 32.85M | 0.280 | 35.70M | 0.144 | 0.47× | 0.51× |
| 100,000 | 2.611 | 38.30M | 2.747 | 36.40M | 1.297 | 0.50× | 0.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.146 | 0.124 | 0.85× |
| 1 | 5 | 0.392 | 0.359 | 0.92× |
| 1 | 10 | 0.561 | 0.571 | 1.02× |
| 10 | 1 | 0.064 | 0.058 | 0.90× |
| 10 | 5 | 0.261 | 0.264 | 1.01× |
| 10 | 10 | 0.568 | 0.585 | 1.03× |
| 100 | 1 | 0.064 | 0.058 | 0.91× |
| 100 | 5 | 0.295 | 0.272 | 0.92× |
| 100 | 10 | 0.585 | 0.572 | 0.98× |
| 1,000 | 1 | 0.090 | 0.074 | 0.83× |
| 1,000 | 5 | 0.290 | 0.311 | 1.08× |
| 1,000 | 10 | 0.565 | 0.747 | 1.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
