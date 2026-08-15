# MathCot benchmark (`numpy.tan reciprocal` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.17M | 0.011 | 87.97M | 0.025 | 2.01× | 2.23× |
| 10,000 | 0.135 | 74.11M | 0.131 | 76.55M | 0.146 | 1.08× | 1.12× |
| 100,000 | 1.331 | 75.16M | 1.279 | 78.19M | 1.272 | 0.96× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.097 | 0.64× |
| 1 | 5 | 0.288 | 0.289 | 1.00× |
| 1 | 10 | 0.378 | 0.590 | 1.56× |
| 10 | 1 | 0.039 | 0.061 | 1.58× |
| 10 | 5 | 0.203 | 0.279 | 1.38× |
| 10 | 10 | 0.414 | 0.599 | 1.45× |
| 100 | 1 | 0.043 | 0.064 | 1.48× |
| 100 | 5 | 0.198 | 0.294 | 1.49× |
| 100 | 10 | 0.409 | 0.602 | 1.47× |
| 1,000 | 1 | 0.064 | 0.083 | 1.30× |
| 1,000 | 5 | 0.234 | 0.362 | 1.54× |
| 1,000 | 10 | 0.445 | 0.795 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
