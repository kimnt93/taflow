# MathCot benchmark (`numpy.tan reciprocal` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.13M | 0.013 | 79.03M | 0.024 | 1.68× | 1.87× |
| 10,000 | 0.141 | 70.74M | 0.122 | 81.83M | 0.140 | 0.99× | 1.14× |
| 100,000 | 1.336 | 74.84M | 1.303 | 76.76M | 1.280 | 0.96× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.073 | 0.68× |
| 1 | 5 | 0.343 | 0.290 | 0.85× |
| 1 | 10 | 0.499 | 0.675 | 1.35× |
| 10 | 1 | 0.048 | 0.060 | 1.24× |
| 10 | 5 | 0.215 | 0.278 | 1.30× |
| 10 | 10 | 0.443 | 0.620 | 1.40× |
| 100 | 1 | 0.052 | 0.061 | 1.18× |
| 100 | 5 | 0.237 | 0.291 | 1.23× |
| 100 | 10 | 0.471 | 0.588 | 1.25× |
| 1,000 | 1 | 0.063 | 0.076 | 1.20× |
| 1,000 | 5 | 0.241 | 0.340 | 1.41× |
| 1,000 | 10 | 0.482 | 0.793 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
